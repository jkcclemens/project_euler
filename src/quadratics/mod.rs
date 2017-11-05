use num::{Num, CheckedAdd, CheckedMul, pow, checked_pow};

#[cfg(test)]
mod test;

/// Returns the result of `ax^2 + bx + c`.
pub fn gen_quadratic<T: Num + Clone>(a: T, b: T, c: T, x: T) -> T {
  (a * pow(x.clone(), 2)) + (b * x) + c
}

/// Returns the result of `ax^2 + bx + c` if it does not overflow on any step.
pub fn checked_gen_quadratic<T>(a: &T, b: &T, c: &T, x: &T) -> Option<T>
  where T: Num + CheckedAdd + CheckedMul + Clone
{
  let first_part = checked_pow(x.clone(), 2).and_then(|d| d.checked_mul(a));
  let second_part = b.checked_mul(x);
  let third_part = second_part.and_then(|s| first_part.and_then(|f| f.checked_add(&s)));
  third_part.and_then(|t| t.checked_add(c))
}
