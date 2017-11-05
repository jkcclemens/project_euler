// Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is
// formed as follows:

// 21 22 23 24 25
// 20  7  8  9 10
// 19  6  1  2 11
// 18  5  4  3 12
// 17 16 15 14 13

// It can be verified that the sum of the numbers on the diagonals is 101.

// What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?

extern crate project_euler;

use project_euler::quadratics::gen_quadratic;

fn main() {
  // The four diagonals are represented by four quadratic equations. Sum the values of those four
  // equations from [2, half any dimension of the spiral truncated] and add 1 to get the answer.
  let n = 1001 / 2;
  let diagonals: [Vec<i64>; 4] = [(2..n + 2).map(|x| gen_quadratic(4, -10, 7, x)).collect(),
                  (2..n + 2).map(|x| gen_quadratic(4, -8, 5, x)).collect(),
                  (2..n + 2).map(|x| gen_quadratic(4, -6, 3, x)).collect(),
                  (2..n + 2).map(|x| gen_quadratic(4, -4, 1, x)).collect()];
  let sum: i64 = diagonals.into_iter()
    .flat_map(|x| x)
    .sum();
  println!("{}", sum + 1);
}
