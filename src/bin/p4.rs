// A palindromic number reads the same both ways. The largest palindrome made from the product of
// two 2-digit numbers is 9009 = 91 × 99.

// Find the largest palindrome made from the product of two 3-digit numbers.

extern crate project_euler;

use project_euler::palindrome::is_palindrome;

fn main() {
  let mut palindromes = Vec::new();
  for x in (100..999).rev() {
    for y in (100..999).rev() {
      let z = x * y;
      if is_palindrome(&z.to_string()) {
        palindromes.push(z);
      }
    }
  }

  println!("{}", palindromes.iter().max().unwrap());
}
