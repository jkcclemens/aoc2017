extern crate itertools;

use itertools::Itertools;
use std::fs::File;
use std::io::Read;

fn main() {
  let mut content = String::new();
  File::open("./inputs/day4").unwrap().read_to_string(&mut content).unwrap();
  let parts: Vec<&str> = content.split('\n').filter(|x| !x.is_empty()).collect();

  println!("{}", p1(&parts));
  println!("{}", p2(&parts));
}

fn p1(input: &[&str]) -> usize {
  input.iter().filter(is_valid).count()
}

fn is_valid<T: AsRef<str>>(input: &T) -> bool {
  let parts: Vec<&str> = input.as_ref().split(' ').collect();
  parts.len() == parts.iter().unique().count()
}

fn p2(input: &[&str]) -> usize {
  input.iter().filter(is_valid_p2).count()
}

fn is_valid_p2<T: AsRef<str>>(input: &T) -> bool {
  let parts: Vec<&str> = input.as_ref().split(' ').collect();
  let unique = parts.iter()
    .map(|x| {
      let mut chars: Vec<char> = x.chars().collect();
      chars.sort();
      chars
    })
    .unique()
    .count();
  parts.len() == unique
}

#[cfg(test)]
mod test {
  use is_valid;

  #[test]
  fn check_given() {
    assert!(is_valid("aa bb cc dd ee"));
    assert!(!is_valid("aa bb cc dd aa"));
    assert!(is_valid("aa bb cc dd aaa"));
  }
}
