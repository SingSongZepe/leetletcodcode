mod test;

struct Solution;

use std::collections::BinaryHeap;
use std::collections::BTreeMap;
use std::cmp::PartialOrd;
use std::cmp::Ord;

struct ReverseI32(i32);

impl PartialEq<Self> for ReverseI32 {
	fn eq(&self, other: &Self) -> bool {
		self.0.eq(&other.0)
	}
}

impl PartialOrd for ReverseI32 {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(other.0.cmp(&self.0))
	}
}

impl Ord for ReverseI32 {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		other.0.cmp(&self.0)
	}
}

impl Eq for ReverseI32 {}

impl Solution {
	pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
		let (mut l, mut r) = (0, 0);
		let mut ans = 0;
		let len = nums.len();

		let mut bt = BTreeMap::new();


		while r < len {
			*bt.entry(&nums[r]).or_insert(1) += 1;
			if let (Some((&&mi, _)), Some((&&m, _))) = (bt.first_key_value(), bt.last_key_value()) {
				if m - mi <= limit {
					ans += 1;
				} else {
					while let (Some((&&min, &cmin)), Some((&&max, &cmax))) = (bt.first_key_value(), bt.last_key_value()) {
						if l <= r {
							break;
						}
						if max - min > limit {
							if let Some(value) = bt.get_mut(&nums[l]) {
								if *value == 1 {
									bt.remove(&nums[l]);
								} else {
									*value -= 1;
								}
							}
							l += 1;
						} else {
							ans += 1;
							break;
						}
					}
				}
			}
			r += 1;
		}

		ans
	}
}

struct Solution1;

impl Solution1 {
	pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
		use std::collections::VecDeque;
		let mut min_deque = VecDeque::new();
		let mut max_deque = VecDeque::new();
		let mut left = 0;
		let mut right = 0;
		let mut res = 0;
		while right < nums.len() {
			while !min_deque.is_empty() && nums[right] < nums[*min_deque.back().unwrap()] {
				min_deque.pop_back();
			}
			while !max_deque.is_empty() && nums[right] > nums[*max_deque.back().unwrap()] {
				max_deque.pop_back();
			}
			min_deque.push_back(right);
			max_deque.push_back(right);
			while nums[*max_deque.front().unwrap()] - nums[*min_deque.front().unwrap()] > limit {
				if left == *min_deque.front().unwrap() {
					min_deque.pop_front();
				}
				if left == *max_deque.front().unwrap() {
					max_deque.pop_front();
				}
				left += 1;
			}
			res = res.max(right - left + 1);
			right += 1;
		}
		return res as i32;
	}
}


fn main() {
    println!("Hello, world!");
}
