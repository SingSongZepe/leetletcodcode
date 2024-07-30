mod test;

struct Solution;

use std::collections::HashMap;

// Solution by Hashmap
impl Solution {
	pub fn is_path_crossing(path: String) -> bool {
		let mut hm = HashMap::new();
		let mut x = 0;
		let mut y = 0;
		for c in path.chars() {
			if let Some(v) = hm.get(&(x, y)) {
				if *v {
					return true;
				}
			}
			match c {
				'N' => y += 1,
				'S' => y -= 1,
				'E' => x += 1,
				_ => x -= 1,
			}
			hm.insert((x, y), true);
		}
		false
	}
}

struct Solution1;
use std::collections::HashSet;

// Solution by HashSet, because it's more efficient than Hashmap,
// values of Hashmap in this problem is useless.
impl Solution1 {
	pub fn is_path_crossing(path: String) -> bool {
		let mut x = 0;
		let mut y = 0;
		let mut visited = HashSet::new();
		visited.insert((0, 0));
		for c in path.chars() {
			match c {
				'N' => y += 1,
				'S' => y -= 1,
				'E' => x += 1,
				'W' => x -= 1,
				_ => {}
			}
			if !visited.insert((x, y)) {
				return true;
			}
		}
		false
	}
}

fn main() {
    println!("Hello, world!");
}
