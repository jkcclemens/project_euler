// The following iterative sequence is defined for the set of positive integers:

// n → n/2 (n is even)
// n → 3n + 1 (n is odd)

// Using the rule above and starting with 13, we generate the following sequence:

// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although
// it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at
// 1.

// Which starting number, under one million, produces the longest chain?

// NOTE: Once the chain starts the terms are allowed to go above one million.

extern crate num;

use num::Num;

fn main() {
  let (num, _) = (1..1_000_001u64)
    .map(|x| (x, Collatz::new(x).count()))
    .max_by_key(|&(_, len)| len)
    .unwrap();
  println!("{}", num);
}

struct Collatz<T> {
  x: T,
  done: bool,
  two: T,
  three: T
}

impl<T> Collatz<T>
  where T: Num + Clone
{
  fn new(x: T) -> Self {
    let two = T::one() + T::one();
    let three = two.clone() + T::one();
    Collatz {
      x,
      done: false,
      two,
      three
    }
  }
}

impl<T> Iterator for Collatz<T>
  where T: Num + Clone
{
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.done {
      return None;
    }
    let ret = self.x.clone();
    if ret == T::one() {
      self.done = true;
      return Some(ret);
    }
    if self.x.clone() % self.two.clone() == T::zero() {
      self.x = self.x.clone() / self.two.clone();
    } else {
      self.x = (self.x.clone() * self.three.clone()) + T::one();
    }
    Some(ret)
  }
}
