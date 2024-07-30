mod test;

struct Solution;

	impl Solution {
	pub fn can_win_nim(n: i32) -> bool {
		n % 4 != 0
	}
}

struct Solution1;

impl Solution1 {
	pub fn can_win_nim(n: i32) -> bool {
		n & 3 != 0
	}
}

fn main() {
    println!("Hello, world!");
}
