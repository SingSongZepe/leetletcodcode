mod test;

struct Solution;

use std::collections::HashMap;
impl Solution {
	pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
		nums.iter().enumerate().fold(HashMap::new(), |mut map, (idx1, &num1)| {
			nums.iter().enumerate().skip(idx1 + 1).fold(map, |mut map, (idx2, &num2)| {
				*map.entry(num1 * num2).or_insert(0) += 1;
				map
			})
		})
			.into_iter()
			.filter(|(_, count)| *count > 1)
			.map(|(_, count)| (count * (count - 1) / 2) * 8)
			.sum()
	}
}

fn main() {
    println!("Hello, world!");
}
