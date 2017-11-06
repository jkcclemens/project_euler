use num::{Num, CheckedMul, ToPrimitive, FromPrimitive, range_inclusive, pow};

#[cfg(test)]
mod test;

pub fn factorial<T>(n: &T) -> Option<T>
  where T: Num + CheckedMul + ToPrimitive + PartialOrd + Clone
{
  let two = T::one() + T::one();
  range_inclusive(two, n.clone()).fold(Some(T::one()), |acc, x| acc.and_then(|z| z.checked_mul(&x)))
}

/// Gets the `n`th digit **from the right**, starting at index 0.
pub fn digit<T: Num + Clone>(x: T, n: usize) -> T {
  let two = T::one() + T::one();
  let ten = pow(two.clone(), 3) + two;
  (x / pow(ten.clone(), n)) % ten
}

pub fn digits<T: Num + Clone + ToPrimitive>(x: &T) -> Vec<T>
  where T: Num + ToPrimitive + FromPrimitive + Clone + PartialOrd
{
  if x == &T::zero() {
    return vec![T::zero()];
  }
  let len: T = FromPrimitive::from_f32(x.to_f32().unwrap().log10().floor()).unwrap();
  range_inclusive(T::zero(), len.clone()).map(|n| digit(x.clone(), (len.clone() - n).to_usize().unwrap())).collect()
}

pub fn combine<T: Num + Clone>(x: &[T]) -> T {
  let two = T::one() + T::one();
  let ten = pow(two.clone(), 3) + two;
  x.into_iter()
    .rev()
    .fold((T::one(), T::zero()), |parts, i| (parts.0.clone() * ten.clone(), i.clone() * parts.0 + parts.1))
    .1
}
