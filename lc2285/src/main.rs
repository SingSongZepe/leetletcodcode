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

fn main() {
    println!("Hello, world!");
}
