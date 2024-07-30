mod test;

trait MaxProductDifference<T> {
	fn make(&self) -> T;
}

impl MaxProductDifference<i32> for (i32, i32, i32, i32) {
	fn make(&self) -> i32 {
		(self.0 * self.1) - (self.2 * self.3)
	}
}


struct Solution;

impl Solution {
	pub fn max_product_difference(nums: Vec<i32>) -> i32 {
		let (mut max1, mut max2, mut min1, mut min2) = (i32::MIN, i32::MIN, i32::MAX, i32::MAX);

		for &num in nums.iter() {
			if num > max1 {
				max2 = max1;
				max1 = num;
			} else if num > max2 {
				max2 = num;
			}
			if num < min1 {
				min2 = min1;
				min1 = num;
			} else if num < min2 {
				min2 = num;
			}
		}

		max1 * max2 - min1 * min2
	}

}

struct Solution1;
use std::cmp::Ordering;

impl Solution1 {

	pub fn max_product_difference(nums: Vec<i32>) -> i32 {
		let mut sorted_nums = nums.clone();
		sorted_nums.sort();

		let n = nums.len();
		(sorted_nums[n-1]*sorted_nums[n-2]) - (sorted_nums[0]*sorted_nums[1])
	}

}

struct Solution2;

impl Solution2 {
	pub fn max_product_difference(nums: Vec<i32>) -> i32 {
		nums.iter().fold((i32::MIN, i32::MIN, i32::MAX, i32::MAX), |(mut max1, mut max2, mut min1, mut min2), &num| {
			if num > max1 {
				max2 = max1;
				max1 = num;
			} else if num > max2 {
				max2 = num;
			}
			if num < min1 {
				min2 = min1;
				min1 = num;
			} else if num < min2 {
				min2 = num;
			}
			(max1, max2, min1, min2)
		}).make()
	}
}


fn main() {
    println!("Hello, world!");
}
