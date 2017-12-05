use std::collections::HashMap;

const INPUT: usize = 361_527;

fn main() {
  println!("{}", p1());
  println!("{}", p2());
}

/// Calculate the answer for part 1 of day 3.
///
/// Uses a formula from the OEIS (https://oeis.org/A174344) to determine the coordinate pair
/// representing the input. Then, it uses the Manhattan distance, which when either pair is (0, 0),
/// is just the other pair summed (both parts absolute value).
fn p1() -> i32 {
  let x = x(INPUT as i32);
  let y = y(INPUT as i32);
  x.abs() + y.abs()
}

/// The OEIS formula is shown as recursive, but since it only ever uses the previous value, we can
/// rewrite it as iterative. Otherwise, it would recurse too deeply.
fn x(n: i32) -> i32 {
  (2..n + 1).fold(0, |acc, n| {
    let k = (f64::from(4 * (n - 2) + 1).sqrt().floor() as i32) % 4;
    acc + (f64::from(k) * std::f64::consts::PI / 2.0).sin() as i32
  })
}

/// The same as the x formula, but instead of sin, cos is used.
fn y(n: i32) -> i32 {
  (2..n + 1).fold(0, |acc, n| {
    let k = (f64::from(4 * (n - 2) + 1).sqrt().floor() as i32) % 4;
    acc + (f64::from(k) * std::f64::consts::PI / 2.0).cos() as i32
  })
}

/// Calculate the answer for part 2 of day 3.
///
/// Use a spiral coordinate-point generator and map each point to its value by checking all adjacent
/// points to see if they've been generated yet.
fn p2() -> usize {
  let mut values: HashMap<(i64, i64), usize> = HashMap::new();
  values.insert((0, 0), 1);

  for (x, y) in Spiral::default() {
    let new_value = adjacent(x, y).into_iter()
      .filter(|pair| values.contains_key(pair))
      .map(|pair| values[&pair])
      .sum();
    if new_value > INPUT {
      return new_value;
    }
    values.insert((x, y), new_value);
  }
  unreachable!()
}

/// Get all the possible adjacent points for any (x, y) pair.
fn adjacent(x: i64, y: i64) -> Vec<(i64, i64)> {
  vec![
    (x + 1, y),
    (x + 1, y + 1),
    (x, y + 1),
    (x - 1, y + 1),
    (x - 1, y),
    (x - 1, y - 1),
    (x, y - 1),
    (x + 1, y - 1),
  ]
}

// https://stackoverflow.com/a/33639875 - this guy is da bomb

struct Spiral {
  x: i64,
  y: i64,
  d: i64,
  m: i64
}

impl Default for Spiral {
  fn default() -> Self {
    Spiral {
      x: 0,
      y: 0,
      d: 1,
      m: 1
    }
  }
}

impl Iterator for Spiral {
  type Item = (i64, i64);

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      if 2 * self.x * self.d < self.m {
        self.x += self.d;
        return Some((self.x, self.y));
      }
      if 2 * self.y * self.d < self.m {
        self.y += self.d;
        return Some((self.x, self.y));
      }
      self.d = -self.d;
      self.m += 1;
    }
  }
}

#[cfg(test)]
mod test {
  use {p1, p2};

  #[test]
  fn test_p1() {
    assert_eq!(326, p1());
  }

  #[test]
  fn test_p2() {
    assert_eq!(363010, p2());
  }
}
