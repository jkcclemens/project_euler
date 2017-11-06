use palindrome::is_palindrome;

#[test]
fn test() {
  assert!(is_palindrome("hannah"));
  assert!(!is_palindrome("dog"));
  assert!(is_palindrome("wow"));
}
