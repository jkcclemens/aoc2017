use std::fs::File;
use std::io::{self, Read};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

pub type Result<T> = std::result::Result<T, DataError>;

#[derive(Debug)]
pub enum DataError {
  String(std::string::FromUtf8Error),
  Io(io::Error)
}

impl Display for DataError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", self.description())
  }
}

impl Error for DataError {
  fn description(&self) -> &str {
    match *self {
      DataError::String(ref e) => e.description(),
      DataError::Io(ref e) => e.description()
    }
  }

  fn cause(&self) -> Option<&Error> {
    match *self {
      DataError::String(ref e) => Some(e),
      DataError::Io(ref e) => Some(e)
    }
  }
}

pub struct Data {
  bytes: Vec<u8>
}

impl AsRef<[u8]> for Data {
  fn as_ref(&self) -> &[u8] {
    &self.bytes
  }
}

impl Data {
  pub fn text(self) -> Result<String> {
    String::from_utf8(self.bytes).map_err(DataError::String)
  }

  pub fn lines(self) -> Result<Vec<String>> {
    Ok(self.text()?.lines().map(ToString::to_string).collect())
  }
}

pub fn data(day: usize) -> Result<Data> {
  let mut f = File::open(format!("inputs/day{}", day)).map_err(DataError::Io)?;
  let mut bytes = Vec::new();
  f.read_to_end(&mut bytes).map_err(DataError::Io)?;
  Ok(Data { bytes })
}
