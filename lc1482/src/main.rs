mod test;

struct Solution;

use std::cmp::Ordering;
use std::cmp::Ord;
use std::cmp::Eq;
use std::collections::BinaryHeap;

#[derive(Debug, Eq, PartialEq)]
struct ReverseWrapper(i32);

impl Ord for ReverseWrapper {
	fn cmp(&self, other: &Self) -> Ordering {
		self.0.cmp(&other.0).reverse()
	}
}

impl PartialOrd for ReverseWrapper {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq<i32> for ReverseWrapper {
	fn eq(&self, other: &i32) -> bool {
		self.0 == *other
	}
}

impl Solution {
	pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
		if m * k > bloom_day.len() as i32 {
			return -1;
		} else {
			bloom_day.iter().enumerate().take(k as usize).fold(i32::MAX, |mi, (i, &_)| {
				let mut bh = bloom_day.iter().skip(i).collect::<Vec<_>>().chunks_exact(k as usize).fold(BinaryHeap::new(), |mut bh, chunk| {
					bh.push(ReverseWrapper(**chunk.iter().max().unwrap()));
					bh
				});
				let mut cm = 0;
				for _ in 0..m {
					if let Some(ReverseWrapper(max_day)) = bh.pop() {
						if max_day > cm {
							cm = max_day;
						}
					}
				}
				cm.min(mi)
			})
		}
	}
}


struct Solution1;

impl Solution1 {
	pub fn canPick(bloom_day: &Vec<i32>, m: i32, k: i32, d: i32) -> bool {
		let mut f_count: i32 = 0;
		let mut b_count: i32 = 0;
		for i in 0..bloom_day.len() {
			f_count = if bloom_day[i] <= d { f_count + 1 } else { 0 };
			b_count = if f_count == k { b_count + 1 } else { b_count };
			f_count = if f_count == k { 0 } else { f_count };
		}
		return b_count >= m;
	}
	pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
		let mut l = bloom_day.iter().min().unwrap().clone();
		let mut r = bloom_day.iter().max().unwrap().clone();
		while l <= r {
			let mid: i32 = (l + r) / 2;
			(l, r) = if Self::canPick(&bloom_day, m, k, mid) {
				(l, mid - 1)
			} else {
				(mid + 1, r)
			};
		}
		if !Self::canPick(&bloom_day, m, k, l) {
			return -1;
		}
		return l;
	}
}


struct Solution2;

impl Solution2 {
	pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
		let can_pick = |bloom_day: &Vec<i32>, in_day: i32| {
			bloom_day.iter().fold((0, 0), |(mut curr_bloom, mut curr_bouquet), &day| {
				if day <= in_day {
					curr_bloom += 1;
				} else {
					curr_bloom = 0;
				}
				if curr_bloom == k {
					curr_bouquet += 1;
					curr_bloom = 0;
				}
				(curr_bloom, curr_bouquet)
			}).1 >= m
		};
		let (mut l, mut r) = bloom_day.iter().fold((i32::MAX, i32::MIN), |(l, r), &day| (day.min(l), day.max(r)));
		while l <= r {
			let mid = (l + r) / 2;
			if can_pick(&bloom_day, mid) {
				r = mid - 1;
			} else {
				l = mid + 1;
			}
		}
		if !can_pick(&bloom_day, l) {
			-1
		} else {
			l
		}
	}
}


fn main() {
    println!("Hello, world!");
}
