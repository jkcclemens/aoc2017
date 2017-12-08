const INPUT: &[usize] = &[11, 11, 13, 7, 0, 15, 5, 5, 4, 4, 1, 1, 7, 1, 15, 11];

fn main() {
  println!("{}", p1().1);
  println!("{}", p2());
}

fn p1() -> (Vec<Vec<usize>>, usize) {
  let mut copy = INPUT.to_vec();
  let mut states_seen = vec![copy.clone()];
  let mut iters = 0;
  loop {
    redistribute(&mut copy);
    iters += 1;
    let seen = states_seen.contains(&copy);
    states_seen.push(copy.clone());
    if seen {
      break;
    }
  }
  (states_seen, iters)
}

fn p2() -> usize {
  let (states, iters) = p1();
  let last = &states[iters];
  let first = states.iter().position(|x| x == last).unwrap();
  iters - first
}

fn redistribute(data: &mut [usize]) {
  let mut max_index = data.iter().enumerate().fold(0, |acc, (i, x)| {
    if x > &data[acc] {
      i
    } else {
      acc
    }
  });
  let mut max = 0;
  std::mem::swap(&mut data[max_index], &mut max);
  while max > 0 {
    max_index += 1;
    if max_index >= data.len() {
      max_index = 0;
    }
    data[max_index] += 1;
    max -= 1;
  }
}

#[cfg(test)]
mod test {
  use {p1, p2, redistribute};

  #[test]
  fn test_given() {
    let mut state = vec![0, 2, 7, 0];
    redistribute(&mut state);
    assert_eq!(vec![2, 4, 1, 2], state);
    redistribute(&mut state);
    assert_eq!(vec![3, 1, 2, 3], state);
    redistribute(&mut state);
    assert_eq!(vec![0, 2, 3, 4], state);
    redistribute(&mut state);
    assert_eq!(vec![1, 3, 4, 1], state);
    redistribute(&mut state);
    assert_eq!(vec![2, 4, 1, 2], state);
  }

  #[test]
  fn solutions() {
    assert_eq!(4074, p1().1);
    assert_eq!(2793, p2());
  }
}
