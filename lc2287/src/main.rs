mod test;

struct Solution;

use std::collections::HashMap;

impl Solution {
	// error
	pub fn rearrange_characters(s: String, target: String) -> i32 {
		if s.len() < target.len() {
			return 0;
		}

		let mut freq = HashMap::new();
		for c in target.chars() {
			*freq.entry(c).or_insert(0) += 1;
		}
		let mut cfreq = freq.clone();

		let mut ans = 0;

		for c in s.chars() {
			if let Some(count) = cfreq.get_mut(&c) {
				if *count > 0 {
					*count -= 1;
				}
				if cfreq.values().all(|&x| x == 0) {
					ans += 1;
					cfreq = freq.clone();
				}
			}
		}

		ans
	}
}

struct Solution1;

impl Solution1 {
	// error
	pub fn rearrange_characters(s: String, target: String) -> i32 {
		let freq = |s: String| {
			s.chars().fold([0; 26], |mut acc, c| {
				acc[c as usize - b'a' as usize] += 1;
				acc
			})
		};

		let scount = freq(s);
		let tcount = freq(target);
		scount.iter().zip(tcount.iter())
			.filter(|(_, &tc)| tc != 0)
			.map(|(&sc, &tc)| sc / tc)
			.min()
			.unwrap_or(0)
	}
}

struct Solution2;

impl Solution2 {
	pub fn rearrange_characters(s: String, target: String) -> i32 {
		s.chars().fold([0; 26], |mut acc, c| {
			acc[c as usize - b'a' as usize] += 1;
			acc
		}).iter().zip(target.chars().fold([0; 26], |mut acc, c| {
			acc[c as usize - b'a' as usize] += 1;
			acc
		}).iter())
			.filter(|(_, &tc)| tc != 0)
			.map(|(&sc, &tc)| sc / tc)
			.min()
			.unwrap_or(0)
	}
}

fn main() {
    println!("Hello, world!");
}
