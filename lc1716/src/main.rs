mod test;

struct Solution;

impl Solution {
	pub fn total_money(n: i32) -> i32 {
		let (weeks, days) = (n / 7, n % 7);
		(49 + 7 * weeks) * weeks / 2 + (days + 1 + 2 * weeks) * days / 2
	}
}

struct Solution1;

impl Solution1 {
	pub fn total_money(n: i32) -> i32 {
		(49 + 7 * (n / 7)) * (n / 7) / 2 + ((n % 7) + 1 + 2 * (n / 7)) * (n % 7) / 2
	}
}

fn main() {
    println!("Hello, world!");
}
