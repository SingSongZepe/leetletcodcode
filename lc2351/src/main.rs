mod test;

struct Solution;

use std::collections::HashSet;

impl Solution {
	pub fn repeated_character(s: String) -> char {
		let mut hs = HashSet::new();
		for c in s.chars() {
			if hs.contains(&c) {
				return c;
			}
			hs.insert(c);
		}
		' '
	}
}

fn main() {
    println!("Hello, world!");
}
