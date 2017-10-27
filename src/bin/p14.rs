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
