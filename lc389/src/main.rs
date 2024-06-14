mod test;

struct Solution;

impl Solution {
	pub fn find_the_difference(s: String, t: String) -> char {
		(t.chars().fold(0i32, |acc, c| acc + c as i32)
			- s.chars().fold(0i32, |acc, c| acc + c as i32))
			as u8 as char
	}
}

fn main() {
    println!("Hello, world!");
}
