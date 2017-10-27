// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

// What is the sum of the digits of the number 2^1000?

extern crate num;

use num::BigUint;

fn main() {
  let two: BigUint = 2u32.into();
  let res = num::pow(two, 1000);
  let sum: u32 = res.to_str_radix(10).chars().map(|x| x.to_digit(10).unwrap()).sum();
  println!("{}", sum);
}
