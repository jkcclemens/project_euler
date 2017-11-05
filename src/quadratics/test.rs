use quadratics::{gen_quadratic, checked_gen_quadratic};

#[test]
fn test_gen_quadratic() {
  // ax^2 + bx + c = (1 * 4^2) + (2 * 4) + 3 = 16 + 8 + 3 = 27
  assert_eq!(27, gen_quadratic(1, 2, 3, 4));
  // ax^2 + bx + c = (-4 * 1^2) + (-3 * 1) + 2 = -4 - 3 + 2 = -5
  assert_eq!(-5, gen_quadratic(-4, -3, 2, 1));
}

#[test]
fn test_checked_gen_quadratic() {
  // ax^2 + bx + c = (1 * 4^2) + (2 * 4) + 3 = 16 + 8 + 3 = 27
  assert_eq!(Some(27), checked_gen_quadratic(&1, &2, &3, &4));
  // ax^2 + bx + c = (-4 * 1^2) + (-3 * 1) + 2 = -4 - 3 + 2 = -5
  assert_eq!(Some(-5), checked_gen_quadratic(&-4, &-3, &2, &1));
  // This will overflow a u8.
  assert_eq!(None, checked_gen_quadratic(&254u8, &2, &2, &100));
}
