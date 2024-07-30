use std::cmp::min;

mod test;

struct Solution;

impl Solution {
	pub fn min_swaps(s: String) -> i32 {

		1
	}
}

struct Solution1;

impl Solution1 {
	pub fn min_swaps(s: String) -> i32 {
		let s = s.as_bytes();

		let count_0 = s.iter().filter(|&&c| c == 48).count() as i32;
		let count_1 = s.len() - count_0;

		if (count_0 - count_1).abs() > 1 {
			return -1;
		}

		let mut count_0_odd = 0;
	}
}

struct Solution2;

impl Solution2 {
	const ZERO: u8 = '0' as u8;
	const ONE: u8 = '1' as u8;

	pub fn min_swaps(s: String) -> i32 {
		let s: Vec<u8> = s.bytes().collect();
		let n = s.len();

		let count_zeros = s.iter().filter(|&&c| c == Solution::ZERO).count();
		let count_ones = n - count_zeros;

		if (count_zeros as i32 - count_ones as i32).abs() > 1 {
			return -1;
		}

		fn swaps_needed(s: &Vec<u8>, start_char: u8) -> i32 {
			let mut swaps = 0;
			let mut target = start_char as u8;

			for &c in s {
				if c != target {
					swaps += 1;
				}

				target ^= 1;
			}

			swaps / 2
		}

		if count_zeros > count_ones {
			swaps_needed(&s, Solution::ZERO)
		} else if count_ones > count_zeros {
			swaps_needed(&s, Solution::ONE)
		} else {
			swaps_needed(&s, Solution::ZERO).min(swaps_needed(&s, Solution::ONE))
		}
	}
}

fn main() {
    println!("Hello, world!");
}
