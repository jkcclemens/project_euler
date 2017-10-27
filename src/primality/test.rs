extern crate test;

use primality::{is_prime, Primes};

use std::fs::File;
use std::io::Read;

use num::{BigUint, BigInt};

use self::test::Bencher;

fn get_primes() -> Vec<u32> {
  let mut f = File::open("10k_primes").unwrap();
  let mut content = String::new();
  f.read_to_string(&mut content).unwrap();
  content.split(" ").map(|x| x.trim().parse().unwrap()).collect()
}

#[test]
fn prime_check() {
  let primes = get_primes();
  let checked: Vec<u32> = (0..).filter(is_prime).take(10_000).collect();
  assert_eq!(primes, checked);
}

#[test]
fn prime_struct_check() {
  let primes = get_primes();
  let checked: Vec<u32> = Primes::default().take(10_000).collect();
  assert_eq!(primes, checked);
}

#[test]
fn prime_struct_biguint() {
  let five: Vec<BigUint> = Primes::default().take(5).collect();
  assert_eq!(5, five.len());
}

#[bench]
fn check_10k_primes(b: &mut Bencher) {
  b.iter(|| {
    for i in (3..).step_by(2).take(10_000) {
      is_prime(&i);
    }
  })
}

#[test]
fn check_data_types() {
  assert!(is_prime(&3u8));
  assert!(is_prime(&3u16));
  assert!(is_prime(&3u32));
  assert!(is_prime(&3u64));

  assert!(is_prime(&3i8));
  assert!(is_prime(&3i16));
  assert!(is_prime(&3i32));
  assert!(is_prime(&3i64));

  let u_three: BigUint = 3u8.into();
  assert!(is_prime(&u_three));
  let i_three: BigInt = 3u8.into();
  assert!(is_prime(&i_three));
}
