#![feature(test)]

#[cfg(feature = "with_nom")]
#[macro_use]
extern crate nom;
extern crate adventofcode;

use adventofcode::data;
use std::collections::HashMap;
use std::str::FromStr;

#[cfg(not(feature = "with_nom"))]
mod std_impl;
#[cfg(feature = "with_nom")]
#[allow(unused_imports)]
mod nom_impl;

fn main() {
  let statements = get_statements();
  println!("{}", p1(&statements));
  println!("{}", p2(&statements));
}

fn get_statements() -> Vec<Statement> {
  let lines = data(8).unwrap().lines().unwrap();
  Statement::parse(&lines)
}

fn p1(statements: &[Statement]) -> isize {
  let mut registers = Registers::default();
  for stmt in statements {
    stmt.process(&mut registers);
  }
  registers.vals.into_iter().max_by_key(|&(_, val)| val).unwrap().1
}

fn p2(statements: &[Statement]) -> isize {
  let mut max_value = 0;
  let mut registers = Registers::default();
  for stmt in statements {
    stmt.process(&mut registers);
    let reg_max = registers.vals.iter().max_by_key(|&(_, val)| val).unwrap().1;
    if reg_max > &max_value {
      max_value = *reg_max;
    }
  }
  max_value
}

#[derive(Default)]
pub struct Registers {
  pub vals: HashMap<String, isize>
}

impl Registers {
  pub fn register<S: Into<String>>(&mut self, name: S) -> isize {
    *self.vals.entry(name.into()).or_insert(0)
  }

  pub fn register_mut<S: Into<String>>(&mut self, name: S) -> &mut isize {
    self.vals.entry(name.into()).or_insert(0)
  }
}

#[derive(Debug)]
pub struct Statement {
  pub ins: Instruction,
  pub con: Condition
}

impl Statement {
  pub fn new(ins: Instruction, con: Condition) -> Statement {
    Statement {
      ins,
      con
    }
  }

  pub fn process(&self, registers: &mut Registers) {
    if self.con.process(registers) {
      self.ins.process(registers);
    }
  }
}

#[derive(Debug)]
pub struct Instruction {
  pub name: String,
  pub op: Operation,
  pub amt: isize
}

impl Instruction {
  pub fn new(name: &str, op: Operation, amt: isize) -> Instruction {
    Instruction {
      name: name.into(),
      op,
      amt
    }
  }

  pub fn process(&self, registers: &mut Registers) {
    let r = registers.register_mut(self.name.clone());
    self.op.process(r, self.amt);
  }
}

#[derive(Debug)]
pub enum Operation {
  Increase,
  Decrease
}

impl FromStr for Operation {
  type Err = ();

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    match input {
      "dec" => Ok(Operation::Decrease),
      "inc" => Ok(Operation::Increase),
      _ => Err(())
    }
  }
}

impl Operation {
  pub fn process(&self, reg: &mut isize, amt: isize) {
    match *self {
      Operation::Increase => *reg += amt,
      Operation::Decrease => *reg -= amt
    }
  }
}

#[derive(Debug)]
pub enum Operator {
  LessThan,
  GreaterThan,
  LessThanEqualTo,
  GreaterThanEqualTo,
  Equals,
  NotEquals
}

impl FromStr for Operator {
  type Err = ();

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    match input {
      "<" => Ok(Operator::LessThan),
      ">" => Ok(Operator::GreaterThan),
      "<=" => Ok(Operator::LessThanEqualTo),
      ">=" => Ok(Operator::GreaterThanEqualTo),
      "==" => Ok(Operator::Equals),
      "!=" => Ok(Operator::NotEquals),
      _ => Err(())
    }
  }
}

impl Operator {
  pub fn process(&self, lhs: isize, rhs: isize) -> bool {
    match *self {
      Operator::LessThan => lhs < rhs,
      Operator::GreaterThan => lhs > rhs,
      Operator::LessThanEqualTo => lhs <= rhs,
      Operator::GreaterThanEqualTo => lhs >= rhs,
      Operator::Equals => lhs == rhs,
      Operator::NotEquals => lhs != rhs
    }
  }
}

#[derive(Debug)]
pub struct Condition {
  pub name: String,
  pub op: Operator,
  pub amt: isize
}

impl Condition {
  pub fn new(name: &str, op: Operator, amt: isize) -> Condition {
    Condition {
      name: name.into(),
      op,
      amt
    }
  }

  pub fn process(&self, registers: &mut Registers) -> bool {
    self.op.process(registers.register(self.name.clone()), self.amt)
  }
}

#[cfg(test)]
mod test {
  extern crate test;
  use self::test::Bencher;

  use {p1, p2, get_statements};

  #[test]
  fn solutions() {
    let statements = get_statements();
    assert_eq!(5075, p1(&statements));
    assert_eq!(7310, p2(&statements));
  }

  #[bench]
  fn bench(b: &mut Bencher) {
    use std::fs::File;
    use std::io::Read;
    use Statement;

    let mut f = File::open("inputs/day8").unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();
    let lines: Vec<&str> = content.split('\n').filter(|x| !x.is_empty()).collect();
    b.iter(|| Statement::parse(&lines))
  }
}
