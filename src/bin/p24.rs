// A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation
// of the digits 1, 2, 3 and 4. If all of the permutations are listed numerically or alphabetically,
// we call it lexicographic order. The lexicographic permutations of 0, 1 and 2 are:

// 012   021   102   120   201   210

// What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?

extern crate permutohedron;

use permutohedron::LexicalPermutation;

fn main() {
  let mut nums: Vec<u8> = (0..10).collect();
  // only run 999,999 times, since the first state counts as one permutation
  for _ in 0..999_999 {
    assert!(nums.next_permutation());
  }
  println!("{}", nums.iter().map(ToString::to_string).collect::<Vec<_>>().join(""));
}
