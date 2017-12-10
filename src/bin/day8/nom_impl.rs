use *;
use nom::alpha;

named!(amt<&str, isize>, map_res!(is_a!("-0123456789"), str::parse));
named!(instruction<&str, Instruction>, ws!(do_parse!(
  name: alpha >>
  op: map_res!(alt!(tag!("inc") | tag!("dec")), Operation::from_str) >>
  amt: amt >>
  (Instruction::new(name, op, amt))
)));
named!(condition<&str, Condition>, ws!(do_parse!(
  name: alpha >>
  op: map_res!(is_a!("=!><"), Operator::from_str) >>
  amt: amt >>
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
    input.iter().map(|x| statement(x).expect("invalid statement").1).collect()
  }
}
