// In England the currency is made up of pound, £, and pence, p, and there are eight coins in
// general circulation:

// 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p) and £2 (200p).
// It is possible to make £2 in the following way:

// 1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
// How many different ways can £2 be made using any number of coins?

fn main() {
  let goal: usize = 200;

  let coin_sizes: [usize; 8] = [1, 2, 5, 10, 20, 50, 100, 200];
  let mut poss: [usize; 201] = [0; 201];
  poss[0] = 1;

  for i in 0..coin_sizes.len() {
    #[cfg_attr(feature="cargo-clippy", allow(needless_range_loop))]
    for j in coin_sizes[i]..goal + 1 {
      poss[j] += poss[j - coin_sizes[i]];
    }
  }

  println!("{}", poss[goal]);
}
