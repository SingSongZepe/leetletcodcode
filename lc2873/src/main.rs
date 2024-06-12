mod test;

struct Solution;

impl Solution {
	// error
	pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
		let n = nums.len();
		let mut max_values = vec![0; n];
		let mut stack: Vec<usize> = Vec::new();
		let mut max_val: i64 = 0;

		for i in (0..n).rev() {
			while let Some(&top) = stack.last() {
				let value = (nums[top] - nums[i]) * nums[i];
				if value as i64 > max_val {
					max_val = value as i64;
				}
				stack.pop();
			}
			max_values[i] = max_val;
			stack.push(i);
		}

		*max_values.iter().max().unwrap_or(&0)
	}
}

fn main() {
    println!("Hello, world!");
}
