// n! means n × (n − 1) × ... × 3 × 2 × 1

// For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
// and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

// Find the sum of the digits in the number 100!

extern crate num;

use num::{BigUint, One};

fn main() {
  let (two, one_hundred): (BigUint, BigUint) = (2u8.into(), 100u8.into());
  let product = num::range_inclusive(two, one_hundred).fold(BigUint::one(), |acc, x| acc * x);
  let sum: u32 = product.to_str_radix(10).chars().map(|x| x.to_digit(10).unwrap()).sum();
  println!("{}", sum);
}
