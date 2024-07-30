mod test;

struct Solution;

impl Solution {
	// bad solution
	// time exceeded
	pub fn find_maximized_capital(mut k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
		let mut cp = capital.into_iter().zip(profits.into_iter()).fold(Vec::new(), |mut acc, (c, p)| {
			acc.push((c, p));
			acc
		});
		// cp.sort_by(|a, b| a.0.cmp(&b.0));
		while k > 0 {
			if let Some(a) = cp.iter().filter(|&c| c.0 <= w).max_by_key(|&c| c.1) {
				w += a.1;
				cp.remove(cp.iter().position(|c| c == a).unwrap());
			}
			k -= 1;
		}
		w
	}
}

struct Solution2;

use std::collections::BinaryHeap;

impl Solution2 {
	pub fn find_maximized_capital(mut k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
		let mut cp = capital.into_iter().zip(profits.into_iter()).map(|(c, p)| (c, p)).collect::<Vec<_>>();
		cp.sort_by(|a, b| a.0.cmp(&b.0));

		let mut p = 0;
		let mut bp = BinaryHeap::<i32>::new();

		for _ in 0..k {
			while let Some(&a) = cp.get(p).filter(|a| a.0 <= w) {
				bp.push(a.1);
				p += 1;
			}
			if let Some(max_profit) = bp.pop() {
				w += max_profit;
			}
		}
		w
	}
}




struct Solution1;

struct Project {
	capital: i32,
	profit: i32,
}

impl Solution1 {
	pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
		let mut projects: Vec<Project> = capital
			.into_iter()
			.zip(profits.into_iter())
			.map(|(capital, profit)| Project { capital, profit })
			.collect();
		projects.sort_unstable_by_key(|elem| elem.capital);

		let mut total_profit = w;
		let mut next_index = 0;
		let mut opportunities = BinaryHeap::with_capacity(projects.len());

		for _ in 0..k {
			while let Some(proj) = projects.get(next_index).filter(|proj| proj.capital <= total_profit) {
				opportunities.push(proj.profit);
				next_index += 1;
			}
			if let Some(max_profit) = opportunities.pop() {
				total_profit += max_profit;
			}
		}

		total_profit
	}
}

fn main() {
    println!("Hello, world!");
}
