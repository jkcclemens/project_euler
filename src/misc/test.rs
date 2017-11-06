use misc::{digits, combine};

#[test]
fn test_digits() {
  assert_eq!(vec![0], digits(&0));
  assert_eq!(vec![1], digits(&1));
  assert_eq!(vec![1, 2], digits(&12));
  assert_eq!(vec![1, 2, 3], digits(&123));
  assert_eq!(vec![1, 2, 3, 4], digits(&1234));
}

#[test]
fn test_combine() {
  assert_eq!(0, combine(&[0]));
  assert_eq!(1, combine(&[1]));
  assert_eq!(12, combine(&[1, 2]));
  assert_eq!(123, combine(&[1, 2, 3]));
  assert_eq!(1234, combine(&[1, 2, 3, 4]));
}
