mod test;

struct Solution;

use std::collections::{
	HashMap,
	HashSet,
};

impl Solution {
	pub fn unique_occurrences(arr: Vec<i32>) -> bool {
		let hm = arr.into_iter().fold(HashMap::new(), |mut acc, x| {
			*acc.entry(x).or_insert(0) += 1;
			acc
		});
		let mut hs = HashSet::new();
		for (_, count) in hm.iter() {
			if hs.contains(count) {
				return false;
			} hs.insert(count);
		}
		true
	}
}


struct Solution1;

impl Solution1 {
	pub fn unique_occurrences(arr: Vec<i32>) -> bool {
		arr.iter().fold((HashMap::new(), HashSet::new()), |(mut acc, mut hs), &x| {
			if hs.contains(&{
				let count = acc.entry(x).or_insert(0);
				*count += 1;
				*count - 1
			}) {
				return (acc, hs);
			}
			hs.insert(acc[&x]);
			(acc, hs)
		}).1.is_empty()
	}
}

fn main() {
    println!("Hello, world!");
}
