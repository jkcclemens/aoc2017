#![feature(entry_or_default)]

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() {
  let mut groups = groups(&get_input());
  assert_eq!(1, groups.len());

  let group = groups.remove(0);
  println!("{}", p1(&group));
  println!("{}", p2(&group));
}

fn get_input() -> String {
  let mut f = File::open("inputs/day9").unwrap();
  let mut content = String::new();
  f.read_to_string(&mut content).unwrap();
  content
}

fn p1(group: &Group) -> usize {
  group.score(1)
}

fn p2(group: &Group) -> usize {
  group.count_garbage()
}

#[derive(Debug)]
enum Group {
  Groups(Vec<Group>),
  Garbage(Vec<char>),
  Text(Vec<char>)
}

impl Group {
  fn score(&self, depth: usize) -> usize {
    match *self {
      Group::Garbage(_) => 0,
      Group::Text(_) => depth,
      Group::Groups(ref g) => depth + g.iter().map(|x| x.score(depth + 1)).sum::<usize>()
    }
  }

  fn count_garbage(&self) -> usize {
    match *self {
      Group::Garbage(ref g) => {
        let mut total = 0;
        let mut skip_next = false;
        for c in g {
          if skip_next {
            skip_next = false;
            continue;
          }
          if *c == '!' {
            skip_next = true;
            continue;
          }
          total += 1;
        }
        total
      },
      Group::Groups(ref g) => g.iter().map(|x| x.count_garbage()).sum(),
      _ => 0
    }
  }
}

fn groups(input: &str) -> Vec<Group> {
  let mut enums = Vec::new();
  let mut inner_enums: HashMap<usize, Vec<Group>> = HashMap::new();
  let mut ignore_next = false;
  let mut groups: Vec<usize> = Vec::new();
  let mut garbage: Option<usize> = None;
  let chars: Vec<char> = input.chars().collect();
  for (i, c) in chars.iter().enumerate() {
    if ignore_next {
      ignore_next = false;
      continue;
    }
    let in_garbage = garbage.is_some();
    match *c {
      '!' => ignore_next = true,
      '>' if in_garbage => {
        let g = Group::Garbage(chars[garbage.expect("start of garbage missing") + 1..i].to_vec());
        if groups.is_empty() {
          enums.push(g);
        } else {
          inner_enums.entry(groups[groups.len() - 1]).or_default().push(g);
        }
        garbage = None;
      },
      '<' if !in_garbage => garbage = Some(i),
      '{' if !in_garbage => groups.push(i),
      '}' if !in_garbage => {
        let start = groups.pop().expect(&format!("no open group to match index {}", i));
        let group = if let Some(inner) = inner_enums.remove(&start) {
          Group::Groups(inner)
        } else {
          Group::Text(chars[start + 1..i].to_vec())
        };
        if groups.is_empty() {
          enums.push(group);
        } else {
          inner_enums.entry(groups[groups.len() - 1]).or_default().push(group);
        }
      },
      _ => {}
    }
  }
  enums
}

#[cfg(test)]
mod test {
  use {p1, p2, get_input, groups};

  #[test]
  fn test_solutions() {
    let group = groups(&get_input()).remove(0);
    assert_eq!(12396, p1(&group));
    assert_eq!(6346, p2(&group));
  }

  #[test]
  fn test_scores() {
    assert_eq!(1, groups("{}")[0].score(1));
    assert_eq!(6, groups("{{{}}}")[0].score(1));
    assert_eq!(5, groups("{{},{}}")[0].score(1));
    assert_eq!(16, groups("{{{},{},{{}}}}")[0].score(1));
    assert_eq!(1, groups("{<a>,<a>,<a>,<a>}")[0].score(1));
    assert_eq!(9, groups("{{<ab>},{<ab>},{<ab>},{<ab>}}")[0].score(1));
    assert_eq!(9, groups("{{<!!>},{<!!>},{<!!>},{<!!>}}")[0].score(1));
    assert_eq!(3, groups("{{<a!>},{<a!>},{<a!>},{<ab>}}")[0].score(1));
  }

  #[test]
  fn test_count_garbage() {
    assert_eq!(0, groups("<>")[0].count_garbage());
    assert_eq!(17, groups("<random characters>")[0].count_garbage());
    assert_eq!(3, groups("<<<<>")[0].count_garbage());
    assert_eq!(2, groups("<{!>}>")[0].count_garbage());
    assert_eq!(0, groups("<!!>")[0].count_garbage());
    assert_eq!(0, groups("<!!!>>")[0].count_garbage());
    assert_eq!(10, groups("<{o\"i!a,<{i<a>")[0].count_garbage());
  }
}
