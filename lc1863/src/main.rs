mod test;

struct Solution;

impl Solution {
	pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
		let mut sum = 0;
		Self::recv(&nums, 0, 0, &mut sum);
		sum
	}
	pub fn recv(nums: &Vec<i32>, from: usize, curr_val: i32, sum: &mut i32) {
		if nums.len() == from {
			*sum += curr_val;
			return;
		}
		Self::recv(nums, from + 1, curr_val, sum);
		Self::recv(nums, from + 1, curr_val ^ nums[from], sum);
	}
}

struct Solution1;

impl Solution1 {
	// best algo in math practice
	pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
		nums.iter().fold(0, |x, &y|
			x | y
		) * (1 << (nums.len() - 1))
	}
}

fn main() {
    println!("Hello, world!");
}
