mod test;

struct Solution;

use std::collections::HashMap;

impl Solution {
	pub fn find_permutation_difference(s: String, t: String) -> i32 {
		let x = s.chars().enumerate().map(|(i,c)|(c,i as i32)).collect::<HashMap<char, i32>>();
		t.chars().enumerate().map(|(i,c)|
			{let k = x.get(&c).unwrap();
				(i as i32-k).abs()}
		).sum()
	}
}

struct Solution1;

impl Solution1 {
	pub fn find_permutation_difference(s: String, t: String) -> i32 {

		1
	}
}


fn main() {
    println!("Hello, world!");
}
