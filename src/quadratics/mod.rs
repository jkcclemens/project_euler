use num::{Num, pow};

/// Returns the result of `ax^2 + bx + c`.
pub fn general_quadratic<T: Num + Clone>(a: T, b: T, c: T, x: T) -> T {
  (a * pow(x.clone(), 2)) + (b * x) + c
}
