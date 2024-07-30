mod test;

struct Solution;

impl Solution {
	pub fn missing_number(nums: Vec<i32>) -> i32 {
		nums.iter().fold((nums.len() * (nums.len() + 1) / 2) as i32, |acc, x| acc - x)
	}
}

struct Solution1;

impl Solution1 {
	pub fn missing_number(nums: Vec<i32>) -> i32 {
		((nums.len() * (nums.len() + 1)) >> 1) as i32 - nums.iter().sum::<i32>()
	}
}

fn main() {
    println!("Hello, world!");
}
