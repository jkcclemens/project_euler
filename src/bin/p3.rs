// The prime factors of 13195 are 5, 7, 13 and 29.

// What is the largest prime factor of the number 600851475143 ?

extern crate project_euler;

use project_euler::factors::PrimeFactors;

fn main() {
  println!("{:?}", PrimeFactors::new(600_851_475_143u64).max().unwrap());
}
