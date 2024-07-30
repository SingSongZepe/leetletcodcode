mod test;

struct Solution;

impl Solution {
	pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
		let z = 32 - (left ^ right).leading_zeros();
		left >> z << z
	}
}

fn main() {
    println!("Hello, world!");
}
