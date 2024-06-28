mod test;

struct Solution;

impl Solution {
	pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
		let mut ln = roads.into_iter().fold(vec![0; n as usize], |mut ln, road| {
			ln[road[0] as usize] += 1;
			ln[road[1] as usize] += 1;
			ln
		});
		ln.sort_unstable();
		(1..=n).into_iter().fold(0, |acc, i| acc + i as i64 * ln[i as usize - 1])
	}
}

struct Solution1;

struct SortedVec {
	data: Vec<i32>,
}

impl SortedVec {
	fn new(data: Vec<i32>) -> Self {
		SortedVec { data }
	}

	fn sorted(mut self) -> Self {
		self.data.sort_unstable();
		self
	}

	fn into_vec(self) -> Vec<i32> {
		self.data
	}
}

impl Solution1 {
	pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
		SortedVec::new(roads.into_iter().fold(vec![0; n as usize], |mut ln, road| {
			ln[road[0] as usize] += 1;
			ln[road[1] as usize] += 1;
			ln
		})).sorted().into_vec().into_iter().enumerate().fold(0, |acc, (i, ln)| acc + (i + 1) as i64 * ln as i64)
	}
}


fn main() {
    println!("Hello, world!");
}
