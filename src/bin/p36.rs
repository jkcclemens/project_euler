// The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.

// Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.

// (Please note that the palindromic number, in either base, may not include leading zeros.)

extern crate project_euler;

use project_euler::palindrome::is_palindrome;

fn main() {
  let sum: u64 = (0..1_000_000)
    .filter(|x| is_palindrome(&x.to_string()))
    .filter(|x| is_palindrome(&format!("{:b}", x)))
    .sum();
  println!("{}", sum);
}
