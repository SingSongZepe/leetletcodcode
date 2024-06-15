mod test;

struct Solution;

use std::collections::HashMap;

impl Solution {
	// hash table solution
	// O(n) time
	// O(n) space
	pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
		let hm = nums.iter().fold(HashMap::new(), |mut map, num| {
			*map.entry(k - num).or_insert(0) += 1;
			map
		});
		let mut t = 0;
		for &n in hm.keys().filter(|&&n| n <= k / 2) {
			if let Some(&c1) = hm.get(&n) {
				if n == k - n {
					t += c1 / 2;
				} else if let Some(&c2) = hm.get(&(k - n)) {
					t += c1.min(c2);
				}
			}
		}
		t
	}
}

struct Solution1;

impl Solution1 {
	// hash table solution
	// O(nlogn) time
	// O(1) space
	pub fn max_operations(mut nums: Vec<i32>, k: i32) -> i32 {
		nums.sort_unstable();
		let (mut l, mut r) = (0, nums.len() - 1);
		let mut t = 0;
		while l < r {
			match (nums[l] + nums[r]).cmp(&k) {
				std::cmp::Ordering::Equal => {
					t += 1;
					l += 1;
					r -= 1;
				},
				std::cmp::Ordering::Less => {
					l += 1;
				},
				std::cmp::Ordering::Greater => {
					r -= 1;
				}
			}
		}
		t
	}
}

fn main() {
    println!("Hello, world!");
}
