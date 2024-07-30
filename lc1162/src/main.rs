mod test;

struct Solution;

use std::collections::VecDeque;

impl Solution {
	pub fn max_distance(mut grid: Vec<Vec<i32>>) -> i32 {
		let _DIRECTIONS_: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
		let mut vq = VecDeque::new();
		let (m, n) = (grid.len(), grid[0].len());
		let mut visited = vec![vec![false; n]; m];
		let mut max_depth: i32 = 0;
		let mut get_valid_neighbors = |r: i32, c: i32, depth: usize| {
			let mut res = vec![];
			for (dx, dy) in _DIRECTIONS_ {
				let dr = r + dx;
				let dc = c + dy;
				if dr >= 0 && dr < m as i32 && dc >= 0 && dc < n as i32 {
					res.push((dr as usize, dc as usize, depth+1));
				}
			}
			res
		};

		grid.iter_mut().enumerate().for_each(|(i, row)| {
			row.iter_mut().enumerate().for_each(|(j, val)| {
				if *val == 1 {
					vq.push_back((i, j, 0));
				}
				*val = i32::MAX;
			});
		});

		if vq.is_empty() || vq.len() == m*n {
			return -1;
		}

		while !vq.is_empty() {
			let (r, c, depth) = vq.pop_front().unwrap();
			if visited[r][c] {
				continue;
			}
			Self::extend_with_tuple(&mut vq, get_valid_neighbors(r as i32, c as i32, depth));
			if grid[r][c] > depth as i32 {
				grid[r][c] = depth as i32;
			}
			visited[r][c] = true;
			max_depth = depth as i32;
		}
		max_depth
	}
	fn extend_with_tuple<T, I>(vq: &mut VecDeque<T>, iter: I)
		where
			I: IntoIterator<Item = T>,
	{
		for item in iter {
			vq.push_back(item);
		}
	}
}


struct Solution1;

impl Solution1 {
	pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
		let mut max_depth: i32 = -1;
		let m = grid.len();
		let n = grid[0].len();
		let mut grid = grid;  // Move grid into mutable variable
		let mut vq = VecDeque::new();

		let directions: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

		for i in 0..m {
			for j in 0..n {
				if grid[i][j] == 1 {
					vq.push_back((i as usize, j as usize));
				}
			}
		}

		if vq.is_empty() || vq.len() == m * n {
			return -1;
		}

		while let Some((r, c)) = vq.pop_front() {
			for (dr, dc) in &directions {
				let new_r = (r as i32 + dr) as usize;
				let new_c = (c as i32 + dc) as usize;
				if new_r < m && new_c < n && grid[new_r][new_c] == 0 {
					vq.push_back((new_r, new_c));
					grid[new_r][new_c] = grid[r][c] + 1;
					max_depth = grid[new_r][new_c];
				}
			}
		}
		max_depth - 1
	}
}

fn main() {
    println!("Hello, world!");
}
