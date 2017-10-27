pub mod prime;

#[cfg(test)]
mod test;

pub use self::prime::PrimeFactors;

use num::{CheckedAdd, ToPrimitive, Num};
use itertools::Itertools;
use std::iter::Product;

pub fn factors<T>(n: T) -> Vec<T>
  where T: Num + CheckedAdd + ToPrimitive + PartialOrd + Ord + PartialEq + Clone + Product<T>
{
  let prime_factors: Vec<T> = PrimeFactors::new(n).collect();
  let mut factors = Vec::new();
  factors.push(T::one());
  for i in 1..prime_factors.len() + 1 {
    for f in prime_factors.iter().cloned().combinations(i).map(|x| x.into_iter().product()) {
      factors.push(f);
    }
  }
  factors.sort();
  factors.dedup();
  factors
}
