mod test;

struct Solution;

impl Solution {
	pub fn new_mod(num: i32, m: i32) -> i32 {
		let n = num % m;
		if n < 0 {
			n + m
		} else {
			n
		}
	}
	pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
		let mut pool = vec![0; value as usize];
		for &num in &nums {
			pool[Self::new_mod(num, value) as usize] += 1;
		}
		let mut res = 0;
		loop {
			if pool[res % value as usize] > 0 {
				pool[res % value as usize] -= 1;
				res += 1;
			} else {
				break;
			}
		}
		res as i32
	}
}

fn main() {
    println!("Hello, world!");
}
