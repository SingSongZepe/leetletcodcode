mod test;

struct Solution;

impl Solution {
	pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
		let mut xor = start ^ goal;
		let mut count = 0;
		while xor > 0 {
			count += 1;
			xor &= xor - 1;
		}
		count
	}
}

fn main() {
    println!("Hello, world!");
}
