use std::cmp::max;

mod test;

struct Solution;

macro_rules! mkusize {
    ($i:tt) => {
		$i as usize
	};
}

impl Solution {
	pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
		let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
		let mut m = 0;
		for i in 0..grid.len() {
			for j in 0..grid[0].len() {
				if grid[i][j] == 0 {
					continue;
				}
				let mut max_gold = 0;
				let mut curr_gold = 0;
				Self::bt(&grid, i as i32, j as i32, &mut visited, &mut curr_gold, &mut max_gold);
				m = max(m, max_gold);
			}
		}
		m
	}
	fn bt(grid: &Vec<Vec<i32>>, i: i32, j: i32, visited: &mut Vec<Vec<bool>>, curr_gold: &mut i32, max_gold: &mut i32) {
		*curr_gold += grid[mkusize!(i)][mkusize!(j)];
		for (x, y) in [(i, j+1), (i, j-1), (i+1, j), (i-1, j)] {
			if  x < 0 || y < 0 || x >= grid.len() as i32 || y >= grid[0].len() as i32 {
				continue;
			}
			if grid[mkusize!(x)][mkusize!(y)] == 0 || visited[mkusize!(x)][mkusize!(y)] == true {
				*max_gold = (*curr_gold).max(*max_gold);
				continue;
			}
			visited[mkusize!(i)][mkusize!(j)] = true;
			Self::bt(grid, x, y, visited, curr_gold, max_gold);
			visited[mkusize!(i)][mkusize!(j)] = false;
		}
		*curr_gold -= grid[i as usize][j as usize];
	}
}

struct Solution1;

enum Op {
	Visit(usize, usize, i32),
	Leave(usize, usize),
}
use Op::*;

impl Solution1 {
	pub fn get_maximum_gold(mut grid: Vec<Vec<i32>>) -> i32 {
		let m = grid.len();
		let n = grid[0].len();

		let mut max_gold = 0;
		let mut visited  = [[false; 15]; 15];
		let mut stack    = Vec::new();

		for i in 0..m {

			for j in 0..n {
				if grid[i][j] != 0
					&& ((i == 0 || grid[i - 1][j] == 0)
					||  (j == 0 || grid[i][j - 1] == 0)) {

					stack.push(Visit(i, j, 0));

					while let Some(op) = stack.pop() {
						match op {
							Visit(i, j, mut accm) => {
								let val = grid[i][j];

								if val > 0 && !visited[i][j] {
									accm += val;
									max_gold = max_gold.max(accm);

									visited[i][j] = true;
									stack.push(Leave(i, j));

									if i > 0 {
										stack.push(Visit(i - 1, j, accm));
									}
									if i + 1 < m {
										stack.push(Visit(i + 1, j, accm));
									}
									if j > 0 {
										stack.push(Visit(i, j - 1, accm));
									}
									if j + 1 < n {
										stack.push(Visit(i, j + 1, accm));
									}
								}
							},
							Leave(i, j) => {
								visited[i][j] = false;
							}
						}
					}}}}
		max_gold
	}
}

struct Solution2;

impl Solution2 {
	// doesnt work
	pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
		let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
		grid.iter().enumerate().flat_map(|(i, row)| {
			row.iter().enumerate().filter_map(|(j, &val)| {
				if val == 0 { None } else {
					let mut max_gold = 0;
					let mut curr_gold = 0;
					let mut v = visited.clone();
					Self::bt(&grid, i as i32, j as i32, &mut v, &mut curr_gold, &mut max_gold);
					Some(max_gold)
				}
			})
		}).max().unwrap_or(0)
	}
	fn bt(grid: &Vec<Vec<i32>>, i: i32, j: i32, visited: &mut Vec<Vec<bool>>, curr_gold: &mut i32, max_gold: &mut i32) {
		*curr_gold += grid[mkusize!(i)][mkusize!(j)];
		for (x, y) in [(i, j+1), (i, j-1), (i+1, j), (i-1, j)] {
			if  x < 0 || y < 0 || x >= grid.len() as i32 || y >= grid[0].len() as i32 {
				continue;
			}
			if grid[mkusize!(x)][mkusize!(y)] == 0 || visited[mkusize!(x)][mkusize!(y)] == true {
				*max_gold = (*curr_gold).max(*max_gold);
				continue;
			}
			visited[mkusize!(i)][mkusize!(j)] = true;
			Self::bt(grid, x, y, visited, curr_gold, max_gold);
			visited[mkusize!(i)][mkusize!(j)] = false;
		}
		*curr_gold -= grid[i as usize][j as usize];
	}
}

fn main() {
    println!("Hello, world!");
}
