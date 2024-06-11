mod test;

struct Solution;

use std::collections::HashMap;

impl Solution {
	pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
		let mut res: Vec<_> = items1.into_iter().chain(items2.into_iter()).fold(HashMap::new(), |mut map, item| {
			*map.entry(item[0]).or_insert(0) += item[1];
			map
		}).into_iter().map(|(i, count)| vec![i, count]).collect();
		res.sort_by(|a, b| a[0].cmp(&b[0]));
		res
	}
}

fn main() {
    println!("Hello, world!");
}
