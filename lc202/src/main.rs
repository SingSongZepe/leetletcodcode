mod test;

struct Solution;

use std::collections::HashSet;
impl Solution {
	pub fn is_happy(mut n: i32) -> bool {
		let mut seen = HashSet::<i32>::new();

		while n > 1 {
			let mut sum = 0;
			while n > 0 {
				sum += (n % 10) * (n % 10);
				n /= 10;
			}
			if seen.contains(&sum) {
				return false;
			}
			seen.insert(sum);
			n = sum;
		}
		true
	}
}

struct Solution1;

impl Solution1 {
	pub fn is_happy(mut n: i32) -> bool {
		fn digits(n: i32) -> impl Iterator<Item=i32> {
			std::iter::successors(Some(n), |n| Some(n / 10))
				.take_while(|&x| x > 0)
				.map(|x| x % 10)
		}

		// i32::MAX = 2_147_483_647
		let max = 2 * 2 + 9 * 9 * 9;
		std::iter::successors(Some(n), |&n| Some(digits(n).map(|x| x * x).sum()))
			.take(max)
			.any(|x| x == 1)
	}
}

fn main() {
    println!("Hello, world!");
}
