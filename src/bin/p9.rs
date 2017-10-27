// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

// a^2 + b^2 = c^2
// For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.

// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

fn main() {
  for a in 1..1000u64 {
    for b in 1..1000u64 {
      if a >= b {
        continue;
      }
      let c = ((a.pow(2) + b.pow(2)) as f32).sqrt();
      if c.fract() != 0.0 {
        continue;
      }
      let c = c as u64;
      if b >= c {
        continue;
      }
      if a + b + c == 1000 {
        println!("{}", a * b * c);
        return;
      }
    }
  }
}
