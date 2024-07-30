mod test;

struct Solution;

use std::collections::HashMap;

impl Solution {
	pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
		let mut hm = HashMap::<i32, i32>::new();

		for n in nums {
			hm.entry(n).and_modify(|e| *e += 1).or_insert(1);
		}

		let mut remove_times = 0;
		let mut remainder = 0;
		for (_, v) in hm {
			remove_times += v / 2;
			remainder += v % 2;
		}

		vec![remove_times, remainder]
	}
}

struct Solution1;

impl Solution1 {
	pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
		let mut hm = HashMap::<i32, i32>::new();

		for n in nums {
			*hm.entry(n).or_insert(0) += 1;
		}

		let (mut remove_times, mut remainder) = (0, 0);
		for v in hm.values() {
			remove_times += v / 2;
			remainder += v % 2;
		}

		vec![remove_times, remainder]
	}
}

fn main() {
    println!("Hello, world!");
}
