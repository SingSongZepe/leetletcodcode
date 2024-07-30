mod test;

struct Solution;

impl Solution {
	pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
		let prefix_sum = nums.iter().fold((Vec::new(), 0), |(mut prefix_sum, mut s), &num| {
			s += if num & 1 == 0 { 0 } else { 1 };
			prefix_sum.push(s);
			(prefix_sum, s)
		});
		println!("prefix_sum: {:?}", prefix_sum);

		let leading =

		1
	}
}

fn main() {
    println!("Hello, world!");
}
