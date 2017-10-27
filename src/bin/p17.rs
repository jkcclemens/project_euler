// If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3
// + 5 + 4 + 4 = 19 letters used in total.
// If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many
// letters would be used?
// NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23
// letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out
// numbers is in compliance with British usage.

#![feature(test)]

fn main() {
  let sum: usize = (1..1001u32).map(to_word).map(|x| sanitize(&x).len()).sum();
  println!("{}", sum);
}

fn to_word(mut n: u32) -> String {
  let mut s = String::new();
  if n / 1000 != 0 {
    s.push_str(&to_word(n / 1000));
    s.push_str(" thousand");
    n %= 1000;
  }
  if n / 100 != 0 {
    if !s.is_empty() { s.push_str(" "); }
    s.push_str(&to_word(n / 100));
    s.push_str(" hundred");
    n %= 100;
  }
  if n / 10 > 1 {
    if !s.is_empty() {
      s.push_str(" and ");
    }
    let add = match n / 10 {
      2 => "twenty",
      3 => "thirty",
      4 => "forty",
      5 => "fifty",
      6 => "sixty",
      7 => "seventy",
      8 => "eighty",
      9 => "ninety",
      _ => "" // why doesn't unreachable!() work here
    };
    s.push_str(add);
    n %= 10;
    if n > 0 {
      s.push_str("-");
      s.push_str(&to_word(n));
    }
  } else if n / 10 == 1 {
    if !s.is_empty() { s.push_str(" and "); }
    let add = match n {
      10 => "ten",
      11 => "eleven",
      12 => "twelve",
      13 => "thirteen",
      14 => "fourteen",
      15 => "fifteen",
      16 => "sixteen",
      17 => "seventeen",
      18 => "eighteen",
      19 => "nineteen",
      _ => "" // why doesn't unreachable!() work here
    };
    s.push_str(add);
  } else {
    let add = match n {
      1 => "one",
      2 => "two",
      3 => "three",
      4 => "four",
      5 => "five",
      6 => "six",
      7 => "seven",
      8 => "eight",
      9 => "nine",
      _ => ""
    };
    if !s.is_empty() && !add.is_empty() { s.push_str(" and "); }
    s.push_str(add);
  }
  if s.is_empty() && n == 0 {
    s.push_str("zero");
  }
  s
}

fn sanitize(s: &str) -> String {
  s.replace("-", "").replace(" ", "")
}

#[cfg(test)]
mod test {
  extern crate test;

  use self::test::Bencher;
  use {to_word, sanitize};

  #[test]
  fn given() {
    assert_eq!(23, sanitize(&to_word(342)).len());
    assert_eq!(20, sanitize(&to_word(115)).len());
  }

  #[test]
  fn others() {
    assert_eq!("three thousand four hundred and twenty-three", to_word(3423));
    assert_eq!("zero", to_word(0));
    assert_eq!("one", to_word(1));
    assert_eq!("sixteen", to_word(16));
    assert_eq!("three hundred and eleven", to_word(311));
    assert_eq!("twenty-three thousand four hundred and ninety-nine", to_word(23499));
    assert_eq!("thirty-six", to_word(36));
    assert_eq!("five hundred", to_word(500));
    assert_eq!("nine hundred and ninety-nine thousand nine hundred and ninety-nine", to_word(999_999));
  }

  #[test]
  fn million() {
    assert_eq!("one thousand thousand", to_word(1_000_000));
  }

  #[bench]
  fn max(b: &mut Bencher) {
    b.iter(|| to_word(999_999));
  }

  #[bench]
  fn min(b: &mut Bencher) {
    b.iter(|| to_word(0));
  }

  #[ignore]
  #[bench]
  fn all(b: &mut Bencher) {
    b.iter(|| {
      for i in 0..1_000_000 {
        to_word(i);
      }
    })
  }
}
