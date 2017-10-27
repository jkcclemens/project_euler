use num::{Num, CheckedAdd, ToPrimitive, FromPrimitive, range_step_inclusive};

#[cfg(test)]
mod test;

pub fn is_prime<T>(n: &T) -> bool
  where T: Num + CheckedAdd + PartialOrd + Clone + ToPrimitive + FromPrimitive
{
  let two = T::one() + T::one();
  let three = two.clone() + T::one();
  if n <= &T::one() {
    return false;
  }
  if n.clone() % two.clone() == T::zero() {
    return n == &two;
  }
  let max = T::from_f64(n.to_f64().unwrap().sqrt().floor()).unwrap();
  for i in range_step_inclusive(three, max, two) {
    if n.clone() % i == T::zero() {
      return false;
    }
  }
  true
}

struct UnboundedRangeStep<T> {
  state: T,
  step: T,
  done: bool
}

impl<T> UnboundedRangeStep<T>
  where T: CheckedAdd + PartialOrd + Clone
{
  fn new(state: T, step: T) -> Self {
    Self {
      state,
      step,
      done: false
    }
  }
}

impl<T> Iterator for UnboundedRangeStep<T>
  where T: CheckedAdd + PartialOrd + Clone
{
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.done {
      return None;
    }
    let ret = self.state.clone();
    match self.state.checked_add(&self.step) {
      Some(x) => self.state = x,
      None => self.done = true
    }
    Some(ret)
  }
}

pub struct Primes<T> {
  curr: UnboundedRangeStep<T>,
  two: Option<T>,
  first: bool
}

impl<T> Default for Primes<T>
  where T: Num + CheckedAdd + PartialOrd + Clone + ToPrimitive + FromPrimitive
{
  fn default() -> Self {
    let two = T::one() + T::one();
    let three = two.clone() + T::one();
    Primes {
      curr: UnboundedRangeStep::new(three, two.clone()),
      two: Some(two),
      first: true
    }
  }
}

impl<T> Iterator for Primes<T>
  where T: Num + CheckedAdd + PartialOrd + Clone + ToPrimitive + FromPrimitive
{
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.first {
      self.first = false;
      return self.two.take();
    }
    self.curr.by_ref().find(is_prime)
  }
}
