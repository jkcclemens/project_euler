use num::Num;

#[cfg(test)]
mod test;

pub struct PreciseDivision<T> {
  pub whole: T,
  dividend: T,
  divisor: T,
  last: T,
  ten: T
}

impl<T: Num + Clone> PreciseDivision<T> {
  pub fn new(dividend: T, divisor: T) -> Self {
    let whole = dividend.clone() / divisor.clone();
    let two = T::one() + T::one();
    let ten = ::num::pow(two.clone(), 3) + two;
    PreciseDivision {
      dividend,
      divisor,
      whole: whole.clone(),
      last: whole,
      ten
    }
  }
}

impl<T: Num + Clone> Iterator for PreciseDivision<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    self.dividend = self.dividend.clone() - (self.divisor.clone() * self.last.clone());
    if self.dividend == T::zero() {
      return None;
    }
    self.dividend = self.dividend.clone() * self.ten.clone();
    self.last = self.dividend.clone() / self.divisor.clone();
    Some(self.last.clone())
  }
}
