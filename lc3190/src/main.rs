mod test;

struct Solution;

impl Solution {
	pub fn minimum_operations(nums: Vec<i32>) -> i32 {
		nums.into_iter().map(|x| if x % 3 == 0 { 0 } else {1}).sum()
	}
}

fn main() {
    println!("Hello, world!");
}
