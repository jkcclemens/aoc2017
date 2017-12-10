use *;
use nom::alpha;

named!(instruction<&str, Instruction>, ws!(do_parse!(
  name: alpha >>
  op: alt!(tag!("inc") | tag!("dec")) >>
  op: expr_opt!(Operation::from_str(op)) >>
  amt: is_a!("-0123456789") >>
  amt: expr_res!(amt.parse::<isize>()) >>
  (Instruction::new(name, op, amt))
)));
named!(condition<&str, Condition>, ws!(do_parse!(
  name: alpha >>
  op: is_a!("=!><") >>
  op: expr_opt!(Operator::from_str(op)) >>
  amt: is_a!("-0123456789") >>
  amt: expr_res!(amt.parse::<isize>()) >>
  (Condition::new(name, op, amt))
)));
named!(statement<&str, Statement>, ws!(do_parse!(
  ins: instruction >>
  tag!("if") >>
  con: condition >>
  (Statement::new(ins, con))
)));

impl Statement {
  pub fn parse(input: &[&str]) -> Vec<Statement> {
    input.iter().map(|x| statement(&x).expect("invalid statement").1).collect()
  }
}
