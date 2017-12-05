use std::fs::File;
use std::io::Read;

fn main() {
  let mut content = String::new();
  File::open("./inputs/day5").unwrap().read_to_string(&mut content).unwrap();
  let instructions: Vec<isize> = content.split('\n')
    .filter(|x| !x.is_empty())
    .map(|x| x.parse().unwrap())
    .collect();

  println!("{}", p1(instructions.clone()));
  println!("{}", p2(instructions));
}

fn p1(mut instructions: Vec<isize>) -> usize {
  let mut steps = 0;
  let mut idx = 0;
  loop {
    if idx >= instructions.len() {
      break;
    }
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
  loop {
    if idx >= instructions.len() {
      break;
    }
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
  #[test]
  fn solutions() {
    assert_eq!(360_603, p1());
    assert_eq!(25_347_697, p2());
  }
}
