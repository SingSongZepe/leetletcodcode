mod test;

struct Solution;

impl Solution {
	pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
		nums.iter().enumerate().filter(|(idx, &x)| nums.len() % (idx + 1) == 0).map(|(_, &x)| x*x).sum()
	}
}

fn main() {
    println!("Hello, world!");
}
