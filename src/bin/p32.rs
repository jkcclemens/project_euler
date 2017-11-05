// We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n
// exactly once; for example, the 5-digit number, 15234, is 1 through 5 pandigital.

// The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing multiplicand,
// multiplier, and product is 1 through 9 pandigital.

// Find the sum of all products whose multiplicand/multiplier/product identity can be written as a 1
// through 9 pandigital.

// HINT: Some products can be obtained in more than one way so be sure to only include it once in
// your sum.

extern crate permutohedron;

use permutohedron::LexicalPermutation;
use std::collections::HashSet;

fn main() {
  let mut products = HashSet::new();

  let mut nums: Vec<Kind> = (1..10).map(Kind::Number).collect();
  nums.insert(0, Kind::Multiply);
  nums.insert(0, Kind::Equals);

  while nums.next_permutation() {
    let mult_pos = nums.iter().position(|x| x == &Kind::Multiply).unwrap();
    let eq_pos = nums.iter().position(|x| x == &Kind::Equals).unwrap();

    let multiplicand = &nums[..mult_pos];
    if multiplicand.is_empty() || multiplicand.contains(&&Kind::Equals) {
      continue;
    }
    let multiplicand = combine(multiplicand);

    let multiplier = &nums[mult_pos + 1..eq_pos];
    if multiplier.is_empty() {
      continue;
    }
    let multiplier = combine(multiplier);

    let eq = &nums[eq_pos + 1..];
    if eq.is_empty() {
      continue;
    }
    let eq = combine(eq);

    if multiplicand * multiplier == eq {
      products.insert(eq);
    }
  }

  let sum: i64 = products.iter().sum();
  println!("{}", sum);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
  Multiply,
  Equals,
  Number(i64)
}

impl Kind {
  fn num(&self) -> Option<i64> {
    match *self {
      Kind::Number(i) => Some(i),
      _ => None
    }
  }
}

fn combine(x: &[Kind]) -> i64 {
  x.into_iter()
    .map(|k| k.num().unwrap())
    .rev()
    .fold((1, 0), |parts, i| (parts.0 * 10, i * parts.0 + parts.1))
    .1
}
