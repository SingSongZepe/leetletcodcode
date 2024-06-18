mod test;

struct Solution;

use std::collections::BinaryHeap;
impl Solution {
	pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, mut worker: Vec<i32>) -> i32 {
		let mut dp = difficulty.into_iter().zip(profit.into_iter()).collect::<Vec<(i32, i32)>>();
		dp.sort_unstable_by(|a, b| a.0.cmp(&b.0));
		let dpl = dp.len();
		worker.sort_unstable_by(|a, b| a.cmp(&b));

		let mut heap = BinaryHeap::new();
		let mut profit = 0;
		let mut i = 0;
		for w in worker {
			while i < dpl && dp[i].0 <= w {
				heap.push(dp[i].1);
				i += 1;
			}
			if let Some(p) = heap.peek() {
				profit += p;
			}
		}

		profit
	}
}

struct Solution1;

// use std::collections::BinaryHeap;
impl Solution1 {
	pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, mut worker: Vec<i32>) -> i32 {
		let mut dp = difficulty.into_iter().zip(profit.into_iter()).collect::<Vec<(i32, i32)>>();
		dp.sort_unstable_by(|a, b| a.0.cmp(&b.0));
		let dpl = dp.len();
		worker.sort_unstable_by(|a, b| a.cmp(&b));

		let mut p = 0;
		let mut dpm = 0;
		let mut tprofit = 0;

		for w in worker {
			while p < dpl && dp[p].0 <= w {
				dpm = dpm.max(dp[p].1);
				p += 1;
			}
			tprofit += dpm;
		}
		tprofit
	}
}


struct Solution2;

impl Solution2 {
	pub fn max_profit_assignment(mut difficulty: Vec<i32>, profit: Vec<i32>, mut worker: Vec<i32>) -> i32 {
		let mut d = difficulty.into_iter().enumerate().map(|(i, d)| (i, d)).collect::<Vec<(usize, i32)>>();
		d.sort_unstable_by(|a, b| a.1.cmp(&b.1));
		let dl = d.len();
		worker.sort_unstable();

		let mut p = 0;
		let mut pm = 0;
		let mut tprofit = 0;

		for i in 0..worker.len() {
			while p < dl && d[p].1 <= worker[i] {
				pm = pm.max(profit[d[p].0]);
				p += 1;
			}
			tprofit += pm;
		}
		tprofit
	}
}

fn main() {
    println!("Hello, world!");
}
