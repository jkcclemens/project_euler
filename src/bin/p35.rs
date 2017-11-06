#![feature(slice_rotate)]

// The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and
// 719, are themselves prime.

// There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.

// How many circular primes are there below one million?

extern crate project_euler;

use project_euler::primality::{Primes, is_prime};
use project_euler::{combine, digits};

fn main() {
  let mut circular = Vec::new();
  'prime: for i in Primes::<u64>::default().take_while(|&n| n < 1_000_000) {
    let mut digits = digits(&i);
    let len = digits.len();
    for _ in 0..len - 1 {
      digits.rotate(len - 1);
      if !is_prime(&combine(&digits)) {
        continue 'prime;
      }
    }
    circular.push(i);
  }
  println!("{}", circular.len());
}
