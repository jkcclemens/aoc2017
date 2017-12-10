extern crate adventofcode;
extern crate itertools;

use adventofcode::{Data, data};
use itertools::Itertools;

fn main() {
  let parts = data(4).and_then(Data::lines).unwrap();

  println!("{}", p1(&parts));
  println!("{}", p2(&parts));
}

fn p1<T: AsRef<str>>(input: &[T]) -> usize {
  input.iter().filter(is_valid).count()
}

fn is_valid<T: AsRef<str>>(input: &T) -> bool {
  let parts: Vec<&str> = input.as_ref().split(' ').collect();
  parts.len() == parts.iter().unique().count()
}

fn p2<T: AsRef<str>>(input: &[T]) -> usize {
  input.iter().filter(is_valid_p2).count()
}

fn is_valid_p2<T: AsRef<str>>(input: &T) -> bool {
  let parts: Vec<&str> = input.as_ref().split(' ').collect();
  let unique = parts.iter()
    .map(|x| x.chars().sorted())
    .unique()
    .count();
  parts.len() == unique
}

#[cfg(test)]
mod test {
  use adventofcode::data;
  use {p1, p2, is_valid};

  #[test]
  fn solution() {
    let parts = data(4).unwrap().lines().unwrap();

    assert_eq!(451, p1(&parts));
    assert_eq!(223, p2(&parts));
  }

  #[test]
  fn check_given() {
    assert!(is_valid(&"aa bb cc dd ee"));
    assert!(!is_valid(&"aa bb cc dd aa"));
    assert!(is_valid(&"aa bb cc dd aaa"));
  }
}
