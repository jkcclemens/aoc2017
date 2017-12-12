extern crate adventofcode;

use adventofcode::{Data, data};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

fn main() {
  let connections = connections();
  println!("{}", p1(&connections));
  println!("{}", p2(&connections));
}

fn p1(connections: &HashSet<Connection>) -> usize {
  Connection::all(connections, 0).len()
}

fn p2(connections: &HashSet<Connection>) -> usize {
  let mut groups: HashSet<Vec<usize>> = HashSet::new();
  for i in 0..2000 {
    let mut all = Connection::all(connections, i);
    all.sort();
    groups.insert(all);
  }
  groups.len()
}

fn connections() -> HashSet<Connection> {
  data(12).and_then(Data::lines).unwrap().into_iter()
    .flat_map(Connection::parse)
    .collect()
}

#[derive(Debug)]
struct Connection(usize, usize);

impl Connection {
  fn parse<S: AsRef<str>>(s: S) -> Vec<Connection> {
    let mut split = s.as_ref().split(' ');
    let id: usize = split.next().unwrap().parse().unwrap();
    split
      .skip(1)
      .map(|x| x.trim_matches(','))
      .map(|x| x.parse().unwrap())
      .map(|x| Connection(id, x))
      .collect()
  }

  fn contains(&self, x: usize) -> bool {
    self.0 == x || self.1 == x
  }

  fn same(&self) -> bool {
    self.0 == self.1
  }

  fn other(&self, x: usize) -> usize {
    if self.0 == x {
      self.1
    } else {
      self.0
    }
  }

  fn direct(conns: &HashSet<Connection>, i: usize) -> Vec<usize> {
    conns.iter()
      .filter(|x| x.contains(i) && !x.same())
      .map(|x| x.other(i))
      .collect()
  }

  fn all(conns: &HashSet<Connection>, i: usize) -> Vec<usize> {
    let mut connections = Connection::direct(conns, i);
    let mut all = vec![i];
    loop {
      let mut next = Vec::new();
      for &conn in &connections {
        let next_level: Vec<usize> = Connection::direct(conns, conn).into_iter()
          .filter(|x| x != &i && !connections.contains(x) && !all.contains(x))
          .collect();
        next.extend(next_level);
      }
      all.extend(connections);
      connections = next;
      if connections.is_empty() {
        break;
      }
    }
    all
  }
}

impl PartialEq for Connection {
  fn eq(&self, other: &Connection) -> bool {
    (self.0 == other.0 && self.1 == other.1) ||
    (self.0 == other.1 && self.1 == other.0)
  }
}

impl Eq for Connection {}

impl Hash for Connection {
  fn hash<H: Hasher>(&self, state: &mut H) {
    std::cmp::min(self.0, self.1).hash(state);
    std::cmp::max(self.0, self.1).hash(state);
  }
}

#[cfg(test)]
mod test {
  use *;

  fn test_connections() -> HashSet<Connection> {
    const GROUP: &str = "0 <-> 2
      1 <-> 1
      2 <-> 0, 3, 4
      3 <-> 2, 4
      4 <-> 2, 3, 4
      5 <-> 6
      6 <-> 4, 5";
    GROUP.split('\n').map(|x| x.trim()).flat_map(Connection::parse).collect()
  }

  #[test]
  fn solutions() {
    let connections = connections();
    assert_eq!(239, p1(&connections));
    assert_eq!(215, p2(&connections));
  }

  #[test]
  fn examples() {
    let group = test_connections();
    assert_eq!(6, Connection::all(&group, 0).len());
    assert_eq!(1, Connection::all(&group, 1).len());
  }
}
