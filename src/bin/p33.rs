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
    for den in 10..100 {
      if num >= den {
        continue;
      }
      let mut n_chars: Vec<char> = num.to_string().chars().collect();
      let mut d_chars: Vec<char> = den.to_string().chars().collect();
      let mut share = None;
      for c in &n_chars {
        if d_chars.contains(c) {
          share = Some(*c);
        }
      }
      let share = match share {
        Some(s) if s != '0' => s,
        _ => continue
      };
      n_chars.remove_item(&share).unwrap();
      d_chars.remove_item(&share).unwrap();
      let new_num: u32 = n_chars.iter().collect::<String>().parse().unwrap();
      let new_den: u32 = d_chars.iter().collect::<String>().parse().unwrap();
      if new_den == 0 {
        continue;
      }
      let rat = Ratio::new(num, den);
      if rat == Ratio::new(new_num, new_den) {
        total_ratio = total_ratio * rat;
      }
    }
  }
  println!("{}", total_ratio.denom());
}
