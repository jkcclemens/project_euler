// The sum of the squares of the first ten natural numbers is,

// 1^2 + 2^2 + ... + 10^2 = 385
// The square of the sum of the first ten natural numbers is,

// (1 + 2 + ... + 10)^2 = 552 = 3025
// Hence the difference between the sum of the squares of the first ten natural numbers and the
// square of the sum is 3025 − 385 = 2640.

// Find the difference between the sum of the squares of the first one hundred natural numbers and
// the square of the sum.

fn main() {
  println!("{}", difference(100.0));
}

fn difference(n: f32) -> f32 {
  let s = ((n * (n + 1.0)) / 2.0).powf(2.0);
  let s2 = ((2.0 * n) + 1.0) * (n + 1.0) * n / 6.0;
  s - s2
}
