#![feature(test, iterator_step_by)]

extern crate num;
extern crate itertools;

pub mod primality;
pub mod factors;
pub mod precision;
pub mod quadratics;
pub mod misc;
pub mod palindrome;

pub use misc::*;
