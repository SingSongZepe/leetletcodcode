mod test;

struct Solution;

impl Solution {
	// for element can't be repeated
	pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
		for (idx, &n) in nums.iter().enumerate() {
			if n <= k {
				return nums.len() as i32 - idx as i32;
			}
		}
		0
	}
}

struct Solution1;

use std::collections::HashSet;
impl Solution1 {
	pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
		let mut hs = HashSet::new();
		let mut count = 0;
		for (idx, &n) in nums.iter().rev().enumerate() {
			if n <= k {
				if !hs.contains(&n) {
					hs.insert(n);
					count += 1;
				}
			}
			if count == k as usize {
				return idx as i32 + 1;
			}
		}
		0
	}
}


fn main() {
    println!("Hello, world!");
}
