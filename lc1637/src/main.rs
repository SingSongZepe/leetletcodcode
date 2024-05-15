mod test;

struct Solution;

impl Solution {
	pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
		points.sort_unstable();
		let first = points[0][0];
		points.into_iter().fold((first, 0), |(prev, m), p| {
			(p[0], m.max(p[0] - prev))
		}).1
	}
}

struct Solution1;

impl Solution1 {
	pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
		points.sort_unstable();
		let max_width = points.windows(2).map(|w| w[1][0] - w[0][0]).max().unwrap_or(0);
		max_width
	}
}

struct Solution2;

use std::collections::BTreeSet;
impl Solution2 {
	pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
		points.into_iter()
			.map(|v| v[0])
			.collect::<BTreeSet<i32>>()
			.into_iter()
			.collect::<Vec<i32>>()
			.windows(2)
			.map(|x| x[1] - x[0])
			.max()
			.unwrap_or(0)
	}
}

fn main() {
    println!("Hello, world!");
}
