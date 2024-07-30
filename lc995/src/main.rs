mod test;

struct Solution;

impl Solution {
	pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
		let mut nums: Vec<i32> = nums;
		let k: usize = k as usize;
		let mut count = 0;
		let mut flip = 0;
		if k > nums.len() {
			return -1;
		}
		for i in 0..nums.len(){
			let i: usize = i as usize;
			if i >= k {
				flip ^= 1 - nums[i-k];
			}
			if nums[i] == flip {
				if i + k > nums.len() {
					return -1;
				}
				nums[i] = 0;
				flip ^= 1;
				count += 1;
			} else {
				nums[i] = 1;
			}
		}

		count
	}
}

fn main() {
    println!("Hello, world!");
}
