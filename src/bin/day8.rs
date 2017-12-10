use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
  let statements = get_statements();
  println!("{}", p1(&statements));
  println!("{}", p2(&statements));
}

fn get_statements() -> Vec<Statement> {
  let mut f = File::open("inputs/day8").unwrap();
  let mut content = String::new();
  f.read_to_string(&mut content).unwrap();
  let lines = content.split('\n').filter(|x| !x.is_empty()).collect();
  Statement::parse(lines)
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
struct Registers {
  vals: HashMap<String, isize>
}

impl Registers {
  fn register<S: Into<String>>(&mut self, name: S) -> isize {
    *self.vals.entry(name.into()).or_insert(0)
  }

  fn register_mut<S: Into<String>>(&mut self, name: S) -> &mut isize {
    self.vals.entry(name.into()).or_insert(0)
  }
}

#[derive(Debug)]
struct Statement {
  reg: String,
  ins: Instruction,
  op: Operation
}

impl Statement {
  fn parse(input: Vec<&str>) -> Vec<Statement> {
    let mut vec = Vec::new();
    for line in input {
      let parts: Vec<&str> = line.split(' ').collect();
      let stmt_reg = parts[0].to_string();
      let stmt_amt: isize = parts[2].parse().unwrap();
      let stmt_ins = Instruction::parse(parts[1], stmt_amt);
      let op_reg = parts[4].to_string();
      let op_op = Operator::parse(parts[5]);
      let op_amt: isize = parts[6].parse().unwrap();
      let stmt = Statement {
        reg: stmt_reg,
        ins: stmt_ins,
        op: Operation {
          reg: op_reg,
          op: op_op,
          val: op_amt
        }
      };
      vec.push(stmt);
    }
    vec
  }

  fn process(&self, registers: &mut Registers) {
    if self.op.process(registers) {
      let r = registers.register_mut(self.reg.clone());
      self.ins.process(r);
    }
  }
}

#[derive(Debug)]
enum Instruction {
  Increase(isize),
  Decrease(isize)
}

impl Instruction {
  fn parse(input: &str, amt: isize) -> Instruction {
    match input {
      "inc" => Instruction::Increase(amt),
      "dec" => Instruction::Decrease(amt),
      _ => panic!("unexpected instruction")
    }
  }

  fn process(&self, reg: &mut isize) {
    match *self {
      Instruction::Increase(amt) => *reg += amt,
      Instruction::Decrease(amt) => *reg -= amt
    }
  }
}

#[derive(Debug)]
enum Operator {
  LessThan,
  GreaterThan,
  LessThanEqualTo,
  GreaterThanEqualTo,
  Equals,
  NotEquals
}

impl Operator {
  fn parse(input: &str) -> Operator {
    match input {
      "<" => Operator::LessThan,
      ">" => Operator::GreaterThan,
      "<=" => Operator::LessThanEqualTo,
      ">=" => Operator::GreaterThanEqualTo,
      "==" => Operator::Equals,
      "!=" => Operator::NotEquals,
      _ => panic!("unexpected operator")
    }
  }

  fn process(&self, lhs: isize, rhs: isize) -> bool {
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
struct Operation {
  reg: String,
  op: Operator,
  val: isize
}

impl Operation {
  fn process(&self, registers: &mut Registers) -> bool {
    self.op.process(registers.register(self.reg.clone()), self.val)
  }
}

#[cfg(test)]
mod test {
  use {p1, p2, get_statements};

  #[test]
  fn solutions() {
    let statements = get_statements();
    assert_eq!(5075, p1(&statements));
    assert_eq!(7310, p2(&statements));
  }
}
