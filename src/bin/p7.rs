// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is
// 13.

// What is the 10 001st prime number?

extern crate project_euler;

use project_euler::primality::is_prime;

fn main() {
  println!("{}", (0..).filter(is_prime).nth(10_000).unwrap());
}
