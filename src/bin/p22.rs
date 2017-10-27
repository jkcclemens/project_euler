// Using names.txt (right click and 'Save Link/Target As...'), a 46K text file containing over
// five-thousand first names, begin by sorting it into alphabetical order. Then working out the
// alphabetical value for each name, multiply this value by its alphabetical position in the list to
// obtain a name score.

// For example, when the list is sorted into alphabetical order, COLIN, which is worth 3 + 15 + 12 +
// 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a score of 938 × 53 = 49714.

// What is the total of all the name scores in the file?

use std::fs::File;
use std::io::Read;

fn main() {
  let mut names = get_names();
  names.sort();
  let sum: u64 = names.into_iter()
    .enumerate()
    .map(|(i, name)| {
      let sum: u64 = name.chars().map(|x| x as u64 - 64).sum();
      (i as u64 + 1) * sum
    })
    .sum();
  println!("{}", sum);
}

fn get_names() -> Vec<String> {
  let mut f = File::open("p022_names.txt").unwrap();
  let mut content = String::new();
  f.read_to_string(&mut content).unwrap();
  content.split(',').map(|x| &x[1..x.len() - 1]).map(ToOwned::to_owned).collect()
}
