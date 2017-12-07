extern crate itertools;

use itertools::Itertools;
use std::fs::File;
use std::io::Read;

fn main() {
  let bottom = get_bottom();
  println!("{}", p1(&bottom));
  println!("{}", p2(&bottom));
}

fn get_bottom() -> Program {
  let mut f = File::open("inputs/day7").unwrap();
  let mut content = String::new();
  f.read_to_string(&mut content).unwrap();
  let lines = content.split('\n').filter(|x| !x.is_empty()).collect();
  let programs = Program::from_data(lines);
  programs.into_iter().max_by_key(|x| x.holding_depth()).unwrap()
}

fn p1(bottom: &Program) -> &str {
  &bottom.name
}

fn p2(bottom: &Program) -> usize {
  let mut unbalanced = bottom;
  while let Some(ub) = unbalanced.holding.iter().find(|x| !x.is_balanced()) {
    unbalanced = ub;
  }
  let weights: Vec<(usize, usize)> = unbalanced.holding.iter()
    .map(|h| (h.weight, h.combined_weight()))
    .collect();
  for chunk in weights.chunks(3) {
    let (a, b, c) = (chunk[0].1, chunk[1].1, chunk[2].1);
    let (aa, ab, ac) = (chunk[0].0, chunk[1].0, chunk[2].0);
    if a != b && a != c { // a is bad weight
      return if a > b { aa - (a - b) } else { aa + (b - a) };
    } else if b != a && b != c { // b is bad weight
      return if b > a { ab - (b - a) } else { ab + (a - b) };
    } else if c != a && c != b { // c is bad weight
      return if c > a { ac - (c - a) } else { ac + (a - c) };
    }
  }
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
  fn from_data(input: Vec<&str>) -> Vec<Program> {
    let internal_programs: Vec<(String, usize, Vec<String>)> = input.into_iter()
      .map(|line| {
        let parts: Vec<&str> = line.split(' ').collect();
        let name = parts[0].to_string();
        let weight = &parts[1];
        let weight: usize = weight[1..weight.len() - 1].parse().unwrap();
        let holding = if parts.len() >= 4 {
          let together = parts[3..].join(" ");
          together.split(", ").map(ToOwned::to_owned).collect()
        } else {
          Vec::new()
        };
        (name, weight, holding)
      })
      .collect();
    internal_programs.iter()
      .map(|p| convert(&internal_programs, p.0.clone(), p.1, &p.2))
      .collect()
  }

  fn holding_depth(&self) -> usize {
    if self.holding.is_empty() {
      return 0;
    }
    self.holding.iter().fold(1, |acc, h| acc + h.holding_depth())
  }

  fn combined_weight(&self) -> usize {
    if self.holding.is_empty() {
      return self.weight;
    }

    let held: usize = self.holding.iter().map(|h| h.combined_weight()).sum();
    held + self.weight
  }

  fn is_balanced(&self) -> bool {
    if self.holding.is_empty() {
      return true;
    }

    self.holding.iter().map(|h| h.combined_weight()).all_equal()
  }
}

fn convert(in_programs: &[(String, usize, Vec<String>)], name: String, weight: usize, holding: &[String]) -> Program {
  if holding.is_empty() {
    return Program::new(name, weight, Vec::new());
  }
  let holding: Vec<Program> = holding.iter()
    .map(|h| in_programs.iter().find(|ip| &ip.0 == h).unwrap())
    .map(|h| convert(in_programs, h.0.clone(), h.1, &h.2))
    .collect();
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
