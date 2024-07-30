mod test;

struct Solution;

use std::collections::{BTreeMap};

struct BiMap<K, V> {
	map: BTreeMap<K, V>,
	reverse_map: BTreeMap<V, K>,
}

impl<K: Ord + Clone, V: Ord + Clone> BiMap<K, V> {
	fn new() -> Self {
		BiMap {
			map: BTreeMap::new(),
			reverse_map: BTreeMap::new(),
		}
	}

	fn insert(&mut self, key: K, value: V) {
		self.map.insert(key.clone(), value.clone());
		self.reverse_map.insert(value, key);
	}

	fn get_value(&self, key: &K) -> Option<&V> {
		self.map.get(key)
	}

	fn get_key(&self, value: &V) -> Option<&K> {
		self.reverse_map.get(value)
	}
}

impl Solution {
	pub fn word_pattern(pattern: String, s: String) -> bool {
		let ss = s.split_whitespace().collect::<Vec<&str>>();
		if ss.len()!= pattern.len() {
			return false;
		}
		let mut map = BiMap::<char, &str>::new();
		for (i, c) in pattern.chars().enumerate() {
			if let Some(&s) = map.get_value(&c) { // a -> dog
				if let Some(&ch) = map.get_key(&ss[i]) { // check if dog -> a
					if ch != c {
						return false;
					}
				} else {
					return false;
				}
			} else {
				if let Some(&c) = map.get_key(&ss[i]) { // if dog -> a, but cannot find a -> dog
					return false;
				} else { // add a -> dog, dog -> a
					map.insert(c, ss[i]);
				}
			}
		}
		true
	}
}




fn main() {
    println!("Hello, world!");
}
