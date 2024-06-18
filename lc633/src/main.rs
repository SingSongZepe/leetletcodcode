mod test;

struct Solution;

impl Solution {
	pub fn judge_square_sum(c: i32) -> bool {
		for i in 0..=(c as f64).sqrt() as i32 {
			let j = (c as f64 - i as f64 * i as f64).sqrt() as i32;
			if i * i + j * j == c {
				return true;
			}
		} false
	}
}


struct Solution2;

use std::cmp::Ordering;
impl Solution2 {
	pub fn judge_square_sum(c: i32) -> bool {
		let (mut left, mut right, c) = (0, (c as f64).sqrt() as i32, c as i64);
		while left <= right {
			match (left as i64 * left as i64 + right as i64 * right as i64).cmp(&c) {
				Ordering::Equal => {
					return true;
				},
				Ordering::Less => {
					left += 1;
				},
				Ordering::Greater => {
					right -= 1;
				}
			}
		} false
	}
}


struct Solution1;


impl Solution1 {
	pub fn judge_square_sum(c: i32) -> bool {
		let mut a: i64 = 0;
		let mut b: i64 = (c as f64).sqrt() as i64;
		while a <= b {
			let s = a * a + b * b;
			match s.cmp(&(c as i64)) {
				Ordering::Equal => {
					return true;
				}
				Ordering::Less => {
					a += 1;
				}
				Ordering::Greater => {
					b -= 1;
				}
			}
		}
		false
	}
}

fn main() {
    println!("Hello, world!");
}
