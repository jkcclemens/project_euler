// 145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.

// Find the sum of all numbers which are equal to the sum of the factorial of their digits.

// Note: as 1! = 1 and 2! = 2 are not sums they are not included.

extern crate project_euler;

use project_euler::{factorial, digits};

fn main() {
  let sum: u64 = (3..354_295u64)
    .filter(|&i| i == digits(&i).into_iter().map(|n| factorial(&n).unwrap()).sum())
    .sum();
  println!("{}", sum);
}
