// By starting at the top of the triangle below and moving to adjacent numbers on the row below, the maximum total from top to bottom is 23.

// 3
// 7 4
// 2 4 6
// 8 5 9 3

// That is, 3 + 7 + 4 + 9 = 23.

// Find the maximum total from top to bottom of the triangle below:

use std::cmp::max;

fn main() {
  let triangle: [Vec<u8>; 15] = [
    vec![75],
    vec![95, 64],
    vec![17, 47, 82],
    vec![18, 35, 87, 10],
    vec![20, 4, 82, 47, 65],
    vec![19, 1, 23, 75, 3, 34],
    vec![88, 2, 77, 73, 7, 63, 67],
    vec![99, 65, 4, 28, 6, 16, 70, 92],
    vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
    vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
    vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
    vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
    vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
    vec![63, 66, 4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
    vec![4, 62, 98, 27, 23, 9, 70, 98, 73, 93, 38, 53, 60, 4, 23]
  ];

  let mut sum: u32 = 0;
  let mut pos: usize = 0;
  for (i, row) in triangle.iter().enumerate() {
    if sum == 0 {
      // at the start
      sum += row[pos] as u32;
      continue;
    }
    if triangle.len() > i + 1 {
      // if there's another row
      let next_row = &triangle[i + 1];
      // get the maximum possible value of the three possible values in the next row
      let next_max = max(max(next_row[pos], next_row[pos + 1]), *next_row.get(pos + 2).unwrap_or(&0));
      if next_max == *next_row.get(pos + 2).unwrap_or(&0) && next_max - max(next_row[pos], *next_row.get(pos + 1).unwrap_or(&0)) > (row[pos] as i64 - row[pos + 1] as i64).abs() as u8 {
        // get the cost of taking a loss on this choice and compare it to the net gain from the next
        // choice. if the net gain is more than the loss, move and take the loss, if not, continue
        // also please forgive me for the if statement from hell
        pos += 1;
      } else if row[pos + 1] > row[pos] {
        // choose the bigger one
        pos += 1;
      }
    } else if row[pos + 1] > row[pos] {
      // last row, just check if it's bigger
      pos += 1;
    }
    // println!("{}", row[pos]); // uncomment to see the path (minus the first non-choice)
    sum += row[pos] as u32;
  }
  println!("{}", sum);
}
