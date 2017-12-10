use std::fs::File;
use std::io::{self, Read};

pub struct Data {
  bytes: Vec<u8>
}

impl AsRef<[u8]> for Data {
  fn as_ref(&self) -> &[u8] {
    &self.bytes
  }
}

impl Data {
  pub fn text(self) -> Result<String, std::string::FromUtf8Error> {
    String::from_utf8(self.bytes)
  }

  pub fn lines(self) -> Result<Vec<String>, std::string::FromUtf8Error> {
    Ok(self.text()?.lines().map(ToString::to_string).collect())
  }
}

pub fn data(day: usize) -> io::Result<Data> {
  let mut f = File::open(format!("inputs/day{}", day))?;
  let mut bytes = Vec::new();
  f.read_to_end(&mut bytes)?;
  Ok(Data { bytes })
}
