extern crate adventofcode;
extern crate itertools;

use adventofcode::{Data, data};
use itertools::Itertools;

fn main() {
  let bottom = get_bottom();
  println!("{}", p1(&bottom));
  println!("{}", p2(&bottom));
}

fn get_bottom() -> Program {
  // load the input and split it into lines
  let lines = data(7).and_then(Data::lines).unwrap();
  // turn every line into a program, potentially holding other programs
  // this is probably the most memory-inefficient way to do this
  let programs = Program::from_data(lines);
  // get the program with the highest holding depth, as that is the bottom
  programs.into_iter().max_by_key(Program::holding_depth).unwrap()
}

fn p1(bottom: &Program) -> &str {
  &bottom.name
}

fn p2(bottom: &Program) -> usize {
  // since there's only one unbalanced program, follow the unbalanced branches up to the top
  let mut unbalanced = bottom;
  while let Some(ub) = unbalanced.holding.iter().find(|x| !x.is_balanced()) {
    unbalanced = ub;
  }
  // get all of the programs' own weights and their combined weights in this branch
  let weights: Vec<(usize, usize)> = unbalanced.holding.iter()
    .map(|h| (h.weight, h.combined_weight()))
    .collect();
  // compare the weights in groups of 3 to find the odd one out
  for chunk in weights.chunks(3) {
    // these are the combined weights
    let (a, b, c) = (chunk[0].1, chunk[1].1, chunk[2].1);
    // these are the "actual" weights
    let (aa, ab, ac) = (chunk[0].0, chunk[1].0, chunk[2].0);
    // find the unbalanced program
    // if a is unbalanced
    if a != b && a != c {
      // if a, the unbalanced program, is heavier than b, a balanced program, subtract the
      // difference from a. otherwise, add it to a.
      return if a > b { aa - (a - b) } else { aa + (b - a) };
    // if b is unbalanced
    } else if b != a && b != c { // b is bad weight
      return if b > a { ab - (b - a) } else { ab + (a - b) };
    // if c is unbalanced
    } else if c != a && c != b { // c is bad weight
      return if c > a { ac - (c - a) } else { ac + (a - c) };
    }
  }
  // we are told our input has one unbalanced program
  unreachable!()
}

#[derive(Debug)]
struct Program {
  name: String,
  weight: usize,
  holding: Vec<Program>
}

impl Program {
  fn new(name: String, weight: usize, holding: Vec<Program>) -> Program {
    Program {
      name,
      weight,
      holding
    }
  }
}

impl Program {
  fn from_data<T: AsRef<str>>(input: Vec<T>) -> Vec<Program> {
    let internal_programs: Vec<(String, usize, Vec<String>)> = input.into_iter()
      .map(|line| {
        // split each line by space
        let parts: Vec<&str> = line.as_ref().split(' ').collect();
        // first part is the name
        let name = parts[0].to_string();
        // second part is the weight in parentheses
        let weight: usize = parts[1].trim_matches(|x| x == '(' || x == ')').parse().unwrap();
        // get any held programs
        let holding = if parts.len() >= 4 {
          let together = parts[3..].join(" ");
          together.split(", ").map(ToOwned::to_owned).collect()
        } else {
          Vec::new()
        };
        // return the three parts of any program
        (name, weight, holding)
      })
      .collect();
    // convert the parts into Programs
    internal_programs.iter()
      .map(|p| convert(&internal_programs, p.0.clone(), p.1, &p.2))
      .collect()
  }

  /// Get the depth of the deepest program held by this program.
  fn holding_depth(&self) -> usize {
    if self.holding.is_empty() {
      return 0;
    }
    self.holding.iter().fold(1, |acc, h| acc + h.holding_depth())
  }

  /// Get the weight of this program and the weight of all programs above it.
  fn combined_weight(&self) -> usize {
    if self.holding.is_empty() {
      return self.weight;
    }

    let held: usize = self.holding.iter().map(Program::combined_weight).sum();
    held + self.weight
  }

  /// Check if all the programs above this program weight the same.
  fn is_balanced(&self) -> bool {
    if self.holding.is_empty() {
      return true;
    }

    self.holding.iter().map(Program::combined_weight).all_equal()
  }
}

fn convert(in_programs: &[(String, usize, Vec<String>)], name: String, weight: usize, holding: &[String]) -> Program {
  // if the program isn't holding anything, hooray! make a simple top-level program
  if holding.is_empty() {
    return Program::new(name, weight, Vec::new());
  }
  // otherwise, we'll have to make a program for each held program
  let holding: Vec<Program> = holding.iter()
    // find each held program in the list of parts
    .map(|h| in_programs.iter().find(|ip| &ip.0 == h).unwrap())
    // turn them into Programs using this function
    .map(|h| convert(in_programs, h.0.clone(), h.1, &h.2))
    // get them as a vector
    .collect();
  // make the program
  Program::new(name, weight, holding)
}

#[cfg(test)]
mod test {
  use {p1, p2, get_bottom};

  #[test]
  fn solutions() {
    let bottom = get_bottom();
    assert_eq!("mkxke", p1(&bottom));
    assert_eq!(268, p2(&bottom));
  }
}
