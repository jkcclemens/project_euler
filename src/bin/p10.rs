// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

// Find the sum of all the primes below two million.

extern crate project_euler;

use project_euler::primality::is_prime;

fn main() {
  let sum: u64 = (0..2_000_001u64).filter(is_prime).sum();
  println!("{}", sum);
}
