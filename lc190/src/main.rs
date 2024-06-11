mod test;

struct Solution;

impl Solution {
	pub fn reverse_bits(x: u32) -> u32 {
		// let mut result = 0;
		// for i in 0..32 {
		// 	result |=  ((x >> i) & 1) << (31 - i);
		// }

		// result
		(0..32).fold(0, |acc, i| (acc << 1) | ((x >> i) & 1))
	}
}

struct Solution1;

impl Solution1 {
	pub fn reverse_bits(x: u32) -> u32 {
		x.reverse_bits()
	}
}

fn main() {
    println!("Hello, world!");
}
