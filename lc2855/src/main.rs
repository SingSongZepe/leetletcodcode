use std::env::consts::FAMILY;

mod test;

struct Solution;

impl Solution {
	pub fn minimum_right_shifts(nums: Vec<i32>) -> i32 {
		// if nums.len() == 1 {
		// 	return 0;
		// }
		let mut decreased = nums[nums.len()-1] > nums[0];
		let mut idx = nums.len()-1;
		for i in 0..nums.len()-1 {
			if nums[i] > nums[i+1] {
				if decreased {
					return -1;
				} else {
					decreased = true;
					idx = i;
				}
			}
		}

		if decreased {
			(nums.len()-1-idx) as i32
		} else {
			0
		}
	}
}

fn main() {
    println!("Hello, world!");
}
