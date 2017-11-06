// The number 3797 has an interesting property. Being prime itself, it is possible to continuously
// remove digits from left to right, and remain prime at each stage: 3797, 797, 97, and 7. Similarly
// we can work from right to left: 3797, 379, 37, and 3.

// Find the sum of the only eleven primes that are both truncatable from left to right and right to
// left.

// NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.

extern crate project_euler;

use project_euler::primality::{Primes, is_prime};
use project_euler::{digits, combine};

fn main() {
  let sum: u64 = Primes::default()
    .skip(4)
    .filter(|&x| is_truncatable(x))
    .take(11)
    .sum();
  println!("{}", sum);
}

fn is_truncatable(n: u64) -> bool {
  // assume input is prime
  if n < 10 {
    return false;
  }
  let digits = digits(&n);
  let len = digits.len();
  if !(0..len).map(|i| combine(&digits[i..])).all(|x| is_prime(&x)) {
    return false;
  }
  if !(0..len).map(|i| combine(&digits[..len - i])).all(|x| is_prime(&x)) {
    return false;
  }
  true
}

#[cfg(test)]
mod test {
  use is_truncatable;

  #[test]
  fn truncatable() {
    assert!(is_truncatable(3797));
    assert!(!is_truncatable(7));
    assert!(!is_truncatable(13));
  }
}
