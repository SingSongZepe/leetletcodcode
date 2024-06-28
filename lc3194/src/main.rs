mod test;

struct Solution;

use std::cmp::Ord;

struct SortedVec<T: Ord>(Vec<T>);

impl<T: Ord> SortedVec<T> {
	fn new(v: Vec<T>) -> Self {
		SortedVec(v)
	}

	fn sorted(mut self) -> Self {
		self.0.sort_unstable();
		self
	}

	fn len(&self) -> usize {
		self.0.len()
	}

	fn make_zip(&self) -> impl Iterator<Item = (&T, &T)> {
		self.0.iter().take(self.len() / 2).zip(self.0.iter().rev().take(self.len() / 2))
	}
}

impl Solution {
	pub fn minimum_average(mut nums: Vec<i32>) -> f64 {
		SortedVec::new(nums).sorted().make_zip().map(|(a, b)| (a + b) as f64 / 2.0).fold(f64::MAX, |acc, x| acc.min(x))
	}
}

fn main() {
    println!("Hello, world!");
}
