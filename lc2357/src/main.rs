mod test;

struct Solution;

use std::collections::HashSet;

impl Solution {
	pub fn minimum_operations(nums: Vec<i32>) -> i32 {
		let mut hs = HashSet::new();
		for n in nums {
			if n != 0 {
				hs.insert(n);
			}
		}
		hs.len() as i32
	}
}

struct Solution1;

impl Solution1 {
	pub fn minimum_operations(nums: Vec<i32>) -> i32 {
		let hs = nums.iter().filter(|&n| *n != 0).collect::<HashSet<_>>();
		hs.len() as i32
	}
}


fn main() {
    println!("Hello, world!");
}
