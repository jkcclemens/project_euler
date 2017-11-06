#[cfg(test)]
mod test;

pub fn is_palindrome(s: &str) -> bool {
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
