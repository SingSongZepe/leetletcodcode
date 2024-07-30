mod test;

struct Solution;
use std::collections::BinaryHeap;

impl Solution {
	pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
		let mut wq: Vec<(f64, i32)> = quality.iter().enumerate().zip(wage.iter())
			.map(|((index, &q), (&w))| (w as f64 / q as f64, index as i32))
			.collect();

		wq.sort_by(|a, b| {
			a.partial_cmp(b).unwrap()
		});

		let mut bh: BinaryHeap<i32> = BinaryHeap::new();
		let mut qsum = 0;
		let mut qwmax = 0.0;
		for i in 0..k as usize {
			bh.push(quality[wq[i].1 as usize]);
			qsum += quality[wq[i].1 as usize];
			if wq[i].0 > qwmax {
				qwmax = wq[i].0;
			}
		}
		let mut m: f64 = qsum as f64 * qwmax;
		for i in k as usize..wq.len() {
			let q = quality[wq[i].1 as usize];
			let top = bh.pop().unwrap();
			qwmax = wq[i].0.max(qwmax);
			qsum = qsum + q - top;
			// bh.pop();
			bh.push(q);
			m = (qsum as f64 * qwmax).min(m);
		}
		m
	}
}

// maxRate = max(maxRate, ratio[i].first);
// qualitySum -= maxHeap.top();
// maxHeap.pop();
//
// qualitySum += quality[ratio[i].second];
// maxHeap.push(quality[ratio[i].second]);
// res = min(res, maxRate * qualitySum);

fn main() {
    println!("Hello, world!");
}
