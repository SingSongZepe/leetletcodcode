mod test;

struct Solution;

impl Solution {
	pub fn is_power_of_four(n: i32) -> bool {
		if n <= 0 {
			return false;
		}
		let mut n = n;
		while n % 4 == 0 {
			n /= 4;
		}
		return n == 1;
	}
}

struct Solution1;

impl Solution1 {
	pub fn is_power_of_four(n: i32) -> bool {
		if n <= 0 { return false; };
		n & (n - 1) == 0 && (n & 0b01010101010101010101010101010101) == n
	}
}

fn main() {
    println!("Hello, world!");
}
