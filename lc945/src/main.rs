mod test;

struct Solution;

impl Solution {
	pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
		nums.sort_unstable();
		nums.iter().skip(1).fold((nums[0], 0), |
			(curr, ts),
			&num
		| {
			if curr >= num {
				(curr + 1, ts + curr - num + 1)
			} else {
				(num, ts)
			}
		}).1
	}
}

fn main() {
    println!("Hello, world!");
}
