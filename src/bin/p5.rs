// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any
// remainder.

// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

fn main() {
  for i in 1.. {
    if is_divisible(i) {
      println!("{}", i);
      break;
    }
  }
}

fn is_divisible(n: u32) -> bool {
  (2..21).all(|x| n % x == 0)
}
