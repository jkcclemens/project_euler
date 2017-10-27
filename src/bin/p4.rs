// A palindromic number reads the same both ways. The largest palindrome made from the product of
// two 2-digit numbers is 9009 = 91 × 99.

// Find the largest palindrome made from the product of two 3-digit numbers.

fn main() {
  let mut palindromes = Vec::new();
  for x in (100..999).rev() {
    for y in (100..999).rev() {
      let z = x * y;
      if is_palindrome(&z.to_string()) {
        palindromes.push(z);
      }
    }
  }

  println!("{}", palindromes.iter().max().unwrap());
}

fn is_palindrome(s: &str) -> bool {
  let x = s.len() as f32 / 2.0;
  let first_half: Vec<char> = s[..x.floor() as usize].chars().collect();
  let second_half: Vec<char> = s[x.ceil() as usize..].chars().collect();
  let len = first_half.len();
  if len != second_half.len() {
    return false;
  }
  for i in 0..len {
    if first_half[i] != second_half[len - i - 1] {
      return false;
    }
  }
  true
}
