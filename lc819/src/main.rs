mod test;

struct Solution;

use std::collections::{
	HashMap,
	HashSet,
};
impl Solution {
	pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
		let mut map = HashMap::new();
		let mut word = String::new();
		for c in paragraph.chars().chain(" ".chars()) {
			match c {
				' ' | '!' | '?' |',' | ';' | '.' | '\'' => {
					if !word.is_empty() {
						*map.entry(word.to_lowercase()).or_insert(0) += 1;
						word.clear();
					}
				},
				_ => {
					word.push(c);
				}
			}
		}

		let set = banned.into_iter().collect::<HashSet<_>>();
		map.into_iter().filter(|(k, _)|!set.contains(k)).max_by_key(|(_, v)| *v).map(|(k, _)| k).unwrap()

	}
}

struct Solution1;

impl Solution1 {
	pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
		let set = banned.into_iter().collect::<HashSet<_>>();
		paragraph.chars().chain(" ".chars()).fold((HashMap::<String, i32>::new(), String::new()), |(mut map, mut word), c| {
			match c {
				' ' | '!' | '?' |',' | ';' | '.' | '\'' => {
					if !word.is_empty() {
						*map.entry(word.to_lowercase()).or_insert(0) += 1;
						word.clear();
					}
				},
				_ => {
					word.push(c);
				}
			}
			(map, word)
		}).0.into_iter().filter(|(k, _)| !set.contains(k)).max_by_key(|(_, v)| *v).map(|(k, _)| k).unwrap()
	}
}

fn main() {
    println!("Hello, world!");
}
