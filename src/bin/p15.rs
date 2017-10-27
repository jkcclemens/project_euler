// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.

// How many such routes are there through a 20×20 grid?

extern crate num;

use num::{Num, CheckedMul, ToPrimitive, BigUint, range_inclusive};

fn main() {
  let twenty: BigUint = 20u32.into();
  println!("{}", lattice_paths(&twenty, &twenty).unwrap());
}

fn lattice_paths<T>(n: &T, k: &T) -> Option<T>
  where T: Num + CheckedMul + ToPrimitive + PartialOrd + Clone
{
  // n + k choose n
  let n_plus_k = n.clone() + k.clone();
  factorial(k)
    .and_then(|k| factorial(n).map(|n| k * n))
    .and_then(|b| factorial(&n_plus_k).map(|nk| nk / b))
}

fn factorial<T>(n: &T) -> Option<T>
  where T: Num + CheckedMul + ToPrimitive + PartialOrd + Clone
{
  let two = T::one() + T::one();
  range_inclusive(two, n.clone()).fold(Some(T::one()), |acc, x| acc.and_then(|z| z.checked_mul(&x)))
}
