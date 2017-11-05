#![feature(vec_remove_item)]

// The fraction 49/98 is a curious fraction, as an inexperienced mathematician in attempting to
// simplify it may incorrectly believe that 49/98 = 4/8, which is correct, is obtained by cancelling
// the 9s.

// We shall consider fractions like, 30/50 = 3/5, to be trivial examples.

// There are exactly four non-trivial examples of this type of fraction, less than one in value, and
// containing two digits in the numerator and denominator.

// If the product of these four fractions is given in its lowest common terms, find the value of the
// denominator.

extern crate num;

use num::rational::Ratio;

fn main() {
  let mut total_ratio = Ratio::new(1, 1);

  for num in 10..100 {
    let (num_1, num_2) = (num / 10, num % 10);
    for den in num + 1..100 {
      let (den_1, den_2) = (den / 10, den % 10);
      let (new_num, new_den) = if num_1 == den_2 && den_1 != 0 {
        (num_2, den_1)
      } else if num_2 == den_1 && den_2 != 0 {
        (num_1, den_2)
      } else {
        continue;
      };
      let rat = Ratio::new(num, den);
      if rat == Ratio::new(new_num, new_den) {
        total_ratio = total_ratio * rat;
      }
    }
  }
  println!("{}", total_ratio.denom());
}
