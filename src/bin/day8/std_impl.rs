use *;

impl Statement {
  pub fn parse<T: AsRef<str>>(input: &[T]) -> Vec<Statement> {
    let mut vec = Vec::new();
    for line in input {
      let parts: Vec<&str> = line.as_ref().split(' ').collect();
      let ins_reg = &parts[0];
      let ins_amt: isize = parts[2].parse().unwrap();
      let ins_op = Operation::from_str(parts[1]).expect("invalid operation");
      let op_reg = &parts[4];
      let op_op = Operator::from_str(parts[5]).expect("invalid operator");
      let op_amt: isize = parts[6].parse().unwrap();
      let stmt = Statement::new(
        Instruction::new(ins_reg, ins_op, ins_amt),
        Condition::new(op_reg, op_op, op_amt)
      );
      vec.push(stmt);
    }
    vec
  }
}
