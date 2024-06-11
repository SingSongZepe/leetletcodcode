mod test;

struct Solution;

use std::collections::BinaryHeap;

impl Solution {
	pub fn fill_cups(amount: Vec<i32>) -> i32 {
		let mut bh = BinaryHeap::<i32>::new();
		for &a in &amount {
			bh.push(a);
		}

		let mut seconds = 0;
		let mut total = amount.iter().sum::<i32>();
		while total > 1 {
			let m1 = bh.pop().unwrap() - 1;
			let m2 = bh.pop().unwrap() - 1;
			if m1 >= 0 && m2 >= 0 {
				total -= 2;
				bh.push(m1);
				bh.push(m2);
				seconds += 1;
			} else {
				break;
			}
		}

		if total >= 1 {
			seconds + total
		} else {
			seconds
		}
	}
}

struct Solution1;

impl Solution1 {
	pub fn fill_cups(mut amount: Vec<i32>) -> i32 {
		if amount.iter().sum::<i32>() == 0 {
			return 0;
		}
		let mut bh = BinaryHeap::from(amount);
		let mut seconds = 0;

		while let Some(m1) = bh.pop() {
			if let Some(m2) = bh.pop() {
				let new_m1 = m1 - 1;
				let new_m2 = m2 - 1;
				if new_m1 > 0 {
					bh.push(new_m1);
				}
				if new_m2 > 0 {
					bh.push(new_m2);
				}
				seconds += 1;
			} else {
				// Odd number of cups left, leave one cup
				seconds += m1;
				break;
			}
		}

		seconds
	}
}


fn main() {
    println!("Hello, world!");
}
