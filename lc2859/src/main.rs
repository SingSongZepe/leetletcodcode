mod test;

struct Solution;

impl Solution {
	pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
		nums.into_iter().enumerate().fold(0, |acc, (i, num)| {
			if i.count_ones() as i32 == k {
				acc + num
			} else {
				acc
			}
		})
	}
}

struct Solution1;

impl Solution1 {
	pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
		let k = k as u32;
		let mut dp = vec![0; nums.len()];
		let mut res = 0;
		for i in 0..nums.len() {
			dp[i] = dp[i / 2] + i as u32 % 2;
			if dp[i] == k {
				res += nums[i];
			}
		}
		res
	}

}

fn main() {
    println!("Hello, world!");
}
