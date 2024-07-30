mod test;

struct Solution;

impl Solution {
	pub fn the_maximum_achievable_x(num: i32, t: i32) -> i32 {
		num + 2 * t
	}
}

fn main() {
    println!("Hello, world!");
}
