mod test;

struct Solution;

use std::cmp::Ordering;

impl Solution {
	pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
		rectangles.into_iter().fold((0, 0), |(m, count), r| {
			let w = r[0].min(r[1]);
			match w.cmp(&m) {
				Ordering::Equal => (m, count + 1),
				Ordering::Greater => (w, 1),
				Ordering::Less => (m, count),
			}
		}).1
	}
}

fn main() {
    println!("Hello, world!");
}
