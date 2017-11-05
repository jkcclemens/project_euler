// By starting at the top of the triangle below and moving to adjacent numbers on the row below, the
// maximum total from top to bottom is 23.

// 3
// 7 4
// 2 4 6
// 8 5 9 3

// That is, 3 + 7 + 4 + 9 = 23.

// Find the maximum total from top to bottom in triangle.txt (right click and 'Save Link/Target
// As...'), a 15K text file containing a triangle with one-hundred rows.

use std::cmp::max;
use std::fs::File;
use std::io::Read;

fn main() {
  let mut triangle = get_triangle();

  for i in 1..triangle.len() + 1 {
    let i = triangle.len() - i;
    if i == 0 {
      println!("{}", triangle[i][0]);
    }
    for c in 0..triangle[i].len() - 1 {
      triangle[i - 1][c] += max(triangle[i][c], triangle[i][c + 1]);
    }
  }
}

fn get_triangle() -> Vec<Vec<u16>> {
  let mut f = File::open("p067_triangle.txt").unwrap();
  let mut content = String::new();
  f.read_to_string(&mut content).unwrap();
  let lines: Vec<&str> = content.split('\n').collect();
  let triangle: Vec<Vec<u16>> = lines.iter()
    .map(|x| x.split(' ')
      .filter(|z| !z.is_empty())
      .map(|z| z.parse::<u16>().unwrap())
      .collect::<Vec<u16>>())
    .filter(|x| !x.is_empty())
    .collect();

  assert_eq!(100, triangle.len());
  let mut last = 0;
  for row in &triangle {
    assert_eq!(1, row.len() - last);
    last = row.len();
  }

  triangle
}
