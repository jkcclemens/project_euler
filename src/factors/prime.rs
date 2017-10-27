use num::{Num, CheckedAdd, ToPrimitive, range_inclusive};

pub struct PrimeFactors<T> {
  x: T
}

impl<T> PrimeFactors<T> {
  pub fn new(x: T) -> Self {
    PrimeFactors { x }
  }
}

impl<T> Iterator for PrimeFactors<T>
  where T: Num + CheckedAdd + ToPrimitive + PartialOrd + Clone
{
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.x == T::one() {
      return None;
    }
    let two = T::one() + T::one();
    let f = range_inclusive(two, self.x.clone())
      .find(|f| self.x.clone() % f.clone() == T::zero())
      .expect("not self-divisible?");
    self.x = self.x.clone() / f.clone();
    Some(f)
  }
}
