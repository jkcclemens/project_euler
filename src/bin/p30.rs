// Surprisingly there are only three numbers that can be written as the sum of fourth powers of
// their digits:

// 1634 = 1^4 + 6^4 + 3^4 + 4^4
// 8208 = 8^4 + 2^4 + 0^4 + 8^4
// 9474 = 9^4 + 4^4 + 7^4 + 4^4
// As 1 = 1^4 is not a sum it is not included.

// The sum of these numbers is 1634 + 8208 + 9474 = 19316.

// Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.

extern crate project_euler;
extern crate num;

use project_euler::digits;
use num::pow;

fn main() {
  let sum: u64 = (2..354_295)
    .filter(|&i| i == digits(&i).into_iter().map(|n| pow(n, 5)).sum())
    .sum();
  println!("{}", sum);
}
