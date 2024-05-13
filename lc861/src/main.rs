mod test;

struct Solution;

trait Max<T> {
	fn max_v(&self) -> T;
	fn min_v(&self) -> T;
}

impl Max<i32> for (i32, i32) {
	fn max_v(&self) -> i32 {
		if self.0 > self.1 {
			self.0
		} else {
			self.1
		}
	}
	fn min_v(&self) -> i32 {
		if self.0 > self.1 {
			self.1
		} else {
			self.0
		}
	}
}

impl Solution {
	pub fn matrix_score(grid: Vec<Vec<i32>>) -> i32 {
		(1..grid[0].len()).fold((1 << (grid[0].len() - 1)) * grid.len() as i32, |acc, col_idx| {
			acc +
				(1 << (grid[0].len() - col_idx - 1)) *
				(grid.len() as i32 - grid
					.iter()
					.fold((0i32, 0i32), |(acc1, acc2), row| (acc1 + if row[0] == row[col_idx] { 1 } else { 0 }, acc2 + if row[0] != row[col_idx] { 1 } else { 0 }))
					.min_v()
				)
		})
	}
}

struct Solution1;

impl Solution1 {
	pub fn matrix_score(grid: Vec<Vec<i32>>) -> i32 {
		let rows = grid.len();
		let cols = grid[0].len();
		let mut score: i32 = (1i32 << (cols - 1)) * (rows as i32);
		for col_idx in 1..cols {
			let count = grid
				.iter()
				.fold(0i32, |acc, row| acc + if row[0] == row[col_idx] { 1 } else { 0 });
			score += count.max(rows as i32 - count) * (1 << (cols - col_idx - 1));
		}
		score
	}
}

struct Solution2;

impl Solution2 {
	// error solution from ai
	pub fn matrix_score(grid: Vec<Vec<i32>>) -> i32 {
		let (m, n) = (grid.len(), grid[0].len());
		grid.iter().enumerate().fold(0, |sum, (i, row)| sum + if row[0] == 0 { (1 << (n as i32) - 1) - row.iter().fold(0, |acc, &x| acc * 2 + x) } else { row.iter().fold(0, |acc, &x| acc * 2 + x) })
	}
}

fn main() {
    println!("Hello, world!");
}
