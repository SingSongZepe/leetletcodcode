mod test;

struct Solution;

impl Solution {
	// but poor at memory usage
	pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
		let mut result = vec![vec![0; grid[0].len()-2]; grid.len()-2];
		(1..grid.len()-1).for_each(|i| {
			(1..grid[0].len()-1).for_each(|j| {
				result[i-1][j-1] = Self::max_around(&grid, i, j);
			})
		});
		result
	}
	fn max_around(grid: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
		let mut max_val = grid[i][j];
		for r in i-1..=i+1 {
			for c in j-1..=j+1 {
				if max_val < grid[r][c] {
					max_val = grid[r][c];
				}
			}
		}
		return max_val;
	}
}

struct Solution1;

impl Solution1 {
	// but poor at memory usage
	pub fn largest_local(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
		(1..grid.len()-1).for_each(|i| {
			(1..grid[0].len()-1).for_each(|j| {
				grid[i-1][j-1] = Self::max_around(&grid, i, j);
			})
		});
		grid.truncate(grid.len()-2);
		grid.iter_mut().for_each(|row| row.truncate(row.len()-2));
		grid
	}
	fn max_around(grid: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
		let mut max_val = grid[i][j];
		for r in i-1..=i+1 {
			for c in j-1..=j+1 {
				if max_val < grid[r][c] {
					max_val = grid[r][c];
				}
			}
		}
		return max_val;
	}
}

struct Solution2;

impl Solution2 {
	pub fn largest_local(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
		(1..grid.len() - 1)
			.map(|i| {
				(1..grid[0].len() - 1)
					.map(|j| {
						let max_val = |r: usize, c: usize| -> i32 {
							let mut max_val = grid[i][j];
							for r in i - 1..=i + 1 {
								for c in j - 1..=j + 1 {
									if max_val < grid[r][c] {
										max_val = grid[r][c];
									}
								}
							}
							max_val
						};
						max_val(i, j)
					})
					.collect()
			})
			.collect()
	}
}


fn print_matrix(matrix: &Vec<Vec<i32>>) {
	for row in matrix {
		println!("{:?}", row);
	}
	println!("");
}

fn main() {
    println!("Hello, world!");
}
