mod test;

struct Solution;

impl Solution {
	pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
		let n = n as i64;
		let mut current_sum = 0;
		let mut patches = 0;
		for num in nums.into_iter().map(|x| x as i64).chain(std::iter::once(n)) {
			if current_sum >= n {
				break;
			}
			while num > current_sum + 1  && current_sum < n {
				patches += 1;
				current_sum += current_sum + 1;
			}
			current_sum += num;
		}
		patches
	}
}

fn main() {
    println!("Hello, world!");
}
