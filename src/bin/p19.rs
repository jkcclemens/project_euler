// You are given the following information, but you may prefer to do some research for yourself.

// 1 Jan 1900 was a Monday.
// Thirty days has September,
// April, June and November.
// All the rest have thirty-one,
// Saving February alone,
// Which has twenty-eight, rain or shine.
// And on leap years, twenty-nine.
// A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible
// by 400.
// How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31
// Dec 2000)?

extern crate chrono;

use chrono::{Utc, TimeZone, Datelike, Weekday};

fn main() {
  let mut start = Utc.ymd(1901, 1, 1);
  let end = Utc.ymd(2000, 12, 1);
  let mut sundays = 0;
  while start != end {
    if start.weekday() == Weekday::Sun {
      sundays += 1;
    }
    if start.month() == 12 {
      start = start.with_month(1).unwrap().with_year(start.year() + 1).unwrap();
    } else {
      start = start.with_month(start.month() + 1).unwrap();
    }
  }
  println!("{}", sundays);
}
