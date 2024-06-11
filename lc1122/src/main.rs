mod test;

struct Solution;

use std::collections::HashMap;

impl Solution {
	pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
		let l = arr2.len(); let mut mp: HashMap<i32, usize> = HashMap::new();
		for (idx, &num) in arr2.iter().enumerate() {
			mp.insert(num, idx);
		}

		let mut arr1 = arr1;
		arr1.sort_by_key(|&x| {
			match mp.get(&x) {
				Some(&idx) => idx,
				None => l + x as usize,
			}
		});

		arr1
	}
}

fn main() {
    println!("Hello, world!");
}
