use precision::PreciseDivision;

macro_rules! precision_test {
  ($name:ident, $dividend:expr, $divisor:expr => $whole:expr, $($dec:expr)*) => {
    #[test]
    fn $name() {
      let div = PreciseDivision::new($dividend, $divisor);
      assert_eq!($whole, div.whole);
      let decimals = vec![$($dec),*];
      if decimals.len() > 1 || decimals[0] != 0 {
        assert_eq!(decimals, div.collect::<Vec<_>>());
      }
    }
  }
}

precision_test!(three_quarters, 3, 4 => 0,7 5);
precision_test!(seven_fifths, 7, 5 => 1,4);
precision_test!(one, 1, 1 => 1,0);
