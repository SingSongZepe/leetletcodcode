mod test;

struct Solution;

use std::collections::HashMap;

impl Solution {
	pub fn largest_word_count(messages: Vec<String>, senders: Vec<String>) -> String {

		let mut map: HashMap<String, i32> = HashMap::new();
		senders.into_iter().zip(messages.into_iter()).for_each(|(sender, message)| {
			*map.entry(sender).or_insert(0) += message.len() as i32;
		});
		let mut max_count = 0;
		let mut min_sender = String::new();
		map.into_iter().for_each(|(sender, count)| {
			if count > max_count {
				max_count = count;
				if min_sender.is_empty() || min_sender > sender {
					min_sender = sender;
				}
			}
		});
		min_sender
	}
}

struct Solution1;

impl Solution1 {
	pub fn largest_word_count(messages: Vec<String>, senders: Vec<String>) -> String {
		let generate_map = || {
			let mut map = HashMap::new();
			map.insert(String::new(), 0);
			map
		};
		senders.into_iter().zip(messages.into_iter()).fold((String::new(), generate_map()), |(max_sender, mut map), (sender, message)| {
			*map.entry(sender.clone()).or_insert(0) += message.split(" ").count() as i32;
			if map[&sender] > map[&max_sender] || map[&sender] == map[&max_sender] && sender > max_sender {
				(sender, map)
			} else {
				(max_sender, map)
			}
		}).0
	}
}



fn helper(vstr: Vec<&str>) -> Vec<String> {
	vstr.iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!("Hello, world!");
}
