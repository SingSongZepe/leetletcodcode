mod test;

struct Solution;

impl Solution {
	pub fn hamming_weight(n: i32) -> i32 {
		let mut count = 0;
		let mut n = n;
		while n > 0 {
			count += n & 1;
			n >>= 1;
		}
		count
	}
}

struct Solution1;

impl Solution1 {
	pub fn hamming_weight(n: i32) -> i32 {
		n.count_ones() as i32
	}
}


struct Solution2;

impl Solution2 {
	pub fn hamming_weight(n: i32) -> i32 {
		(0..32).fold(0, |count, i| count + ((n >> i) & 1))
	}
}

fn main() {
    println!("Hello, world!");
}
