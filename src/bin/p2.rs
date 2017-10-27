// Each new term in the Fibonacci sequence is generated by adding the previous two terms. By
// starting with 1 and 2, the first 10 terms will be:
// 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
// By considering the terms in the Fibonacci sequence whose values do not exceed four million, find
// the sum of the even-valued terms.

extern crate fibonacci;

use fibonacci::Fibonacci;

fn main() {
  let f: Fibonacci<u64> = Fibonacci::default();
  let sum: u64 = f.take_while(|&x| x <= 4_000_000).filter(|x| x % 2 == 0).sum();
  println!("{}", sum);
}
