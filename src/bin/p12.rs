extern crate project_euler;
extern crate num;

use num::{CheckedAdd, Zero, One};
use project_euler::factors::factors;

fn main() {
  let num: u64 = TriangleNumbers::default()
    .map(|x| (x, factors(x)))
    .find(|&(_, ref f)| f.len() > 500)
    .map(|(x, _)| x)
    .unwrap();
  println!("{}", num);
}

struct TriangleNumbers<T> {
  index: T,
  last: T
}

impl<T> Default for TriangleNumbers<T>
  where T: CheckedAdd + Zero + One + Clone
{
  fn default() -> Self {
    TriangleNumbers {
      index: T::zero(),
      last: T::zero()
    }
  }
}

impl<T> Iterator for TriangleNumbers<T>
  where T: CheckedAdd + Zero + One + Clone
{
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    let next_index = match self.index.checked_add(&T::one()) {
      Some(i) => i,
      None => return None
    };
    let res = match self.last.checked_add(&next_index) {
      Some(r) => r,
      None => return None
    };
    self.index = next_index;
    self.last = res.clone();
    Some(res)
  }
}
