mod test;

struct Solution;

use std::collections::HashMap;

impl Solution {
	pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
		let a_kind = |ranks: &Vec<i32>| {
			let mut hm = HashMap::new();
			for &r in ranks {
				*hm.entry(r).or_insert(0) += 1;
			}
			(hm.values().any(|&v| v > 2), hm.values().any(|&v| v == 2))
		};
		if suits.iter().all(|&s| s == suits[0]) {
			"Flush".to_string()
		} else {
			let (three, two) = a_kind(&ranks);
			if three {
				"Three of a Kind".to_string()
			} else if two {
				"Pair".to_string()
			} else {
				"High Card".to_string()
			}
		}
	}
}

fn main() {
    println!("Hello, world!");
}
