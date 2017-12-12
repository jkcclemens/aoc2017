extern crate adventofcode;

use adventofcode::{Data, data};
use std::str::FromStr;

fn main() {
  let dirs = get_directions();
  println!("{}", p1(&dirs));
  println!("{}", p2(&dirs));
}

fn p1(dirs: &[Direction]) -> usize {
  CubePoint::default().directions_distance(dirs)
}

fn p2(dirs: &[Direction]) -> usize {
  let start = CubePoint::default();
  let mut end = CubePoint::default();
  let mut max_distance = 0;
  for dir in dirs {
    end += dir;
    let distance = start.distance(&end);
    if distance > max_distance {
      max_distance = distance;
    }
  }
  max_distance
}

fn get_directions() -> Vec<Direction> {
  data(11)
    .and_then(Data::text)
    .unwrap()
    .lines()
    .next()
    .unwrap()
    .split(',')
    .map(FromStr::from_str)
    .collect::<Result<_, _>>()
    .unwrap()
}

#[derive(Debug)]
enum Direction {
  North,
  Northeast,
  Northwest,
  South,
  Southeast,
  Southwest
}

impl FromStr for Direction {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "n" => Ok(Direction::North),
      "ne" => Ok(Direction::Northeast),
      "nw" => Ok(Direction::Northwest),
      "s" => Ok(Direction::South),
      "se" => Ok(Direction::Southeast),
      "sw" => Ok(Direction::Southwest),
      _ => Err(())
    }
  }
}

impl Direction {
  fn mods(&self) -> (i64, i64, i64) {
    match *self {
      Direction::North => (0, 1, -1),
      Direction::Northeast => (1, 0, -1),
      Direction::Northwest => (-1, 1, 0),
      Direction::South => (0, -1, 1),
      Direction::Southeast => (1, -1, 0),
      Direction::Southwest => (-1, 0, 1)
    }
  }
}

#[derive(Debug, Default, Clone)]
struct CubePoint {
  x: i64,
  y: i64,
  z: i64
}

impl CubePoint {
  fn new(x: i64, y: i64, z: i64) -> CubePoint {
    CubePoint { x, y, z }
  }

  fn distance(&self, other: &CubePoint) -> usize {
    ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) as usize / 2
  }

  fn directions_distance(&self, dirs: &[Direction]) -> usize {
    self.distance(&dirs.iter().fold(self.clone(), |acc, x| acc + x))
  }
}

impl From<(i64, i64, i64)> for CubePoint {
  fn from(pts: (i64, i64, i64)) -> Self {
    CubePoint::new(pts.0, pts.1, pts.2)
  }
}

impl<'a> std::ops::Add<&'a Direction> for CubePoint {
  type Output = CubePoint;

  fn add(self, dir: &'a Direction) -> Self::Output {
    let (x, y, z) = dir.mods();
    CubePoint {
      x: self.x + x,
      y: self.y + y,
      z: self.z + z
    }
  }
}

impl<'a> std::ops::AddAssign<&'a Direction> for CubePoint {
  fn add_assign(&mut self, dir: &'a Direction) {
    let (x, y, z) = dir.mods();
    *self = CubePoint {
      x: self.x + x,
      y: self.y + y,
      z: self.z + z
    }
  }
}

#[cfg(test)]
mod test {
  use *;

  #[test]
  fn examples() {
    assert_eq!(3, CubePoint::default().directions_distance(&[Direction::Northeast, Direction::Northeast, Direction::Northeast]));
    assert_eq!(0, CubePoint::default().directions_distance(&[Direction::Northeast, Direction::Northeast, Direction::Southwest, Direction::Southwest]));
    assert_eq!(2, CubePoint::default().directions_distance(&[Direction::Northeast, Direction::Northeast, Direction::South, Direction::South]));
    assert_eq!(3, CubePoint::default().directions_distance(&[Direction::Southeast, Direction::Southwest, Direction::Southeast, Direction::Southwest, Direction::Southwest]));
  }
}
