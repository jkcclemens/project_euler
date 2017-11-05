// Surprisingly there are only three numbers that can be written as the sum of fourth powers of
// their digits:

// 1634 = 1^4 + 6^4 + 3^4 + 4^4
// 8208 = 8^4 + 2^4 + 0^4 + 8^4
// 9474 = 9^4 + 4^4 + 7^4 + 4^4
// As 1 = 1^4 is not a sum it is not included.

// The sum of these numbers is 1634 + 8208 + 9474 = 19316.

// Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.

fn main() {
  let sum: u64 = (2..10_000_000)
    .filter(|&i| i == (0..(i as f32).log10().floor() as u8 + 1).map(|n| digit(i, n).pow(5)).sum())
    .sum();
  println!("{}", sum);
}

/// Gets the `n`th digit **from the right**, starting at index 0, supporting up to 8 digits.
fn digit(x: u64, n: u8) -> u64 {
  const POWERS: [u64; 8] = [1, 10, 100, 1000, 10_000, 100_000, 1_000_000, 10_000_000];
  (x / POWERS[n as usize]) % 10
}
