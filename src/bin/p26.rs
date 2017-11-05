// A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions
// with denominators 2 to 10 are given:

// 1/2	= 	0.5
// 1/3	= 	0.(3)
// 1/4	= 	0.25
// 1/5	= 	0.2
// 1/6	= 	0.1(6)
// 1/7	= 	0.(142857)
// 1/8	= 	0.125
// 1/9	= 	0.(1)
// 1/10	= 	0.1
// Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has a
// 6-digit recurring cycle.

// Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal
// fraction part.

extern crate project_euler;
extern crate itertools;

use project_euler::precision::PreciseDivision;
use itertools::Itertools;

#[allow(unused)]
fn original_algo() {
  // (number, repeat_size)
  let mut longest = (0, 0);
  // loop through [2, 1000)
  'outer: for i in 2..1000 {
    // take chunks starting at 2 and going to infinity
    for size in 2.. {
      let chunks = PreciseDivision::new(1, i).chunks(size);
      // skip the first chunk to hopefully exclude things like 1/6
      let mut iter = chunks.into_iter().skip(1);
      // grab the first chunk from the iterator
      let first: Vec<u32> = match iter.next() {
        Some(x) => x.collect(),
        None => continue 'outer
      };
      // if the chunk isn't as long as the chunk size, it's definitely never repeating
      if first.len() != size {
        continue 'outer;
      }
      // if the next 10 chunks don't equal the first chunk, it's not repeating at this length
      if !iter.take(10).all(|x| x.collect::<Vec<_>>() == first) {
        continue;
      }
      // check that the digits aren't all the same (looking at you, 1/9)
      let first_first = first[0];
      if first[1..].iter().all(|&x| x == first_first) {
        continue 'outer;
      }
      // at this stage, we know this number repeats at this chunk length, so we must break.
      // if the chunk size is longer than the longest repeat so far, replace it
      if size > longest.1 {
        longest = (i, size);
      }
      break;
    }
  }
  println!("{} has a {} digit repeat", longest.0, longest.1);
}

fn main() {
  let mut max = (0, 0);
  for n in 2..1000 {
    let mut rest = 1;
    for _ in 0..n {
      rest = (rest * 10) % n;
    }
    let r0 = rest;
    let mut len = 0;
    loop {
      rest = (rest * 10) % n;
      len += 1;
      if rest == r0 {
        break;
      }
    }
    if len > max.1 {
      max = (n, len);
    }
  }
  println!("{} has a {} digit repeat", max.0, max.1);
}
