// A perfect number is a number for which the sum of its proper divisors is exactly equal to the
// number. For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which
// means that 28 is a perfect number.

// A number n is called deficient if the sum of its proper divisors is less than n and it is called
// abundant if this sum exceeds n.

// As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be
// written as the sum of two abundant numbers is 24. By mathematical analysis, it can be shown that
// all integers greater than 28123 can be written as the sum of two abundant numbers. However, this
// upper limit cannot be reduced any further by analysis even though it is known that the greatest
// number that cannot be expressed as the sum of two abundant numbers is less than this limit.

// Find the sum of all the positive integers which cannot be written as the sum of two abundant
// numbers.

extern crate project_euler;

use project_euler::factors::factors;

// https://oeis.org/A048242 "a(1456) = 20161 is the last term." 28123 is incorrect

fn main() {
  let nums: Vec<u16> = (12..20162).filter(|&x| is_abundant(x)).collect();
  let mut sum_list = [false; 20161 * 2];
  for &i in &nums {
    for &j in &nums {
      sum_list[i as usize + j as usize] = true;
    }
  }
  let sum: u32 = (1..20162).filter(|&x| !sum_list[x as usize]).map(|x| x as u32).sum();
  println!("{}", sum);
}

fn is_abundant(n: u16) -> bool {
  let f = factors(n);
  let sum: u16 = f[..f.len() - 1].iter().sum();
  sum > n
}

#[cfg(test)]
mod test {
  use is_abundant;

  #[test]
  fn given() {
    assert!(is_abundant(12));
    assert!(!is_abundant(28));
  }

  #[test]
  fn others() {
    assert!(!is_abundant(6));
  }
}
