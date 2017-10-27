extern crate fibonacci;
extern crate num;

use fibonacci::Fibonacci;
use num::BigUint;

fn main() {
  let mut f: Fibonacci<BigUint> = Fibonacci::default();
  println!("{}", f.position(|x| x.to_string().len() >= 1000).unwrap() + 2);
}
