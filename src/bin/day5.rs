extern crate adventofcode;

use adventofcode::{Data, data};

fn main() {
  let instructions = instructions();

  println!("{}", p1(instructions.clone()));
  println!("{}", p2(instructions));
}

fn instructions() -> Vec<isize> {
  data(5).and_then(Data::text).unwrap()
    .lines()
    .map(|x| x.parse().unwrap())
    .collect()
}

fn p1(mut instructions: Vec<isize>) -> usize {
  let mut steps = 0;
  let mut idx = 0;
  while idx < instructions.len() {
    let old = instructions[idx];
    instructions[idx] += 1;
    if old < 0 {
      idx -= old.abs() as usize;
    } else {
      idx += old as usize;
    }
    steps += 1;
  }
  steps
}

fn p2(mut instructions: Vec<isize>) -> usize {
  let mut steps = 0;
  let mut idx = 0;
  while idx < instructions.len() {
    let old = instructions[idx];
    if old >= 3 {
      instructions[idx] -= 1;
    } else {
      instructions[idx] += 1;
    }
    if old < 0 {
      idx -= old.abs() as usize;
    } else {
      idx += old as usize;
    }
    steps += 1;
  }
  steps
}

#[cfg(test)]
mod test {
  use {p1, p2, instructions};

  #[test]
  fn solutions() {
    let ins = instructions();

    assert_eq!(360_603, p1(ins.clone()));
    assert_eq!(25_347_697, p2(ins));
  }
}
