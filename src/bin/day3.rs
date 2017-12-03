const INPUT: i32 = 361_527;

fn main() {
  let x = x(INPUT);
  let y = y(INPUT);
  println!("{}", x.abs() + y.abs());
}

fn x(n: i32) -> i32 {
  (2..n + 1).fold(0, |acc, n| {
    let k = (f64::from(4 * (n - 2) + 1).sqrt().floor() as i32) % 4;
    acc + (f64::from(k) * std::f64::consts::PI / 2.0).sin() as i32
  })
}

fn y(n: i32) -> i32 {
  (2..n + 1).fold(0, |acc, n| {
    let k = (f64::from(4 * (n - 2) + 1).sqrt().floor() as i32) % 4;
    acc + (f64::from(k) * std::f64::consts::PI / 2.0).cos() as i32
  })
}
