
mod test;

struct Solution;

use std::collections::VecDeque;

impl Solution {
	pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
		let (rs, cs) = (grid.len(), grid[0].len());
		if grid[0][0] == 1 || grid[rs-1][cs-1] == 1 {
			return 0;
		}

		let (safe_map , mut factor_max) = Self::make_safe_map(&grid, rs, cs);
		print_matrix(&safe_map);
		while factor_max > 0 {
			if Self::connect_by_factor(&safe_map, factor_max, rs, cs) {
				return factor_max;
			}
			factor_max -= 1;
		}
		0
	}
	fn connect_by_factor(safe_map: &Vec<Vec<i32>>, factor: i32, rs: usize, cs: usize) -> bool {
		let mut memo = vec![vec![false; cs]; rs];
		Self::find_path(safe_map, factor, 0, 0, rs, cs, &mut memo)
	}
	fn find_path(safe_map: &Vec<Vec<i32>>, factor: i32, r: i32, c: i32, rs: usize, cs: usize, memo: &mut Vec<Vec<bool>>) -> bool {
		if r >= rs as i32 || c >= cs as i32 || r < 0 || c < 0 || safe_map[r as usize][c as usize] < factor || memo[r as usize][c as usize] {
			return false;
		}
		if r == rs as i32 - 1 && c == cs as i32 - 1 { // reaching the end
			return true;
		}
		memo[r as usize][c as usize] = true;
		// 							right                                                         down
		Self::find_path(safe_map, factor, r, c+1, rs, cs, memo) || Self::find_path(safe_map, factor, r+1, c, rs, cs, memo)
	}
	fn make_safe_map(grid: &Vec<Vec<i32>>, rs: usize, cs: usize) -> (Vec<Vec<i32>>, i32) {
		let mut v = vec![];
		let mut safe_map = vec![vec![i32::MAX; cs]; rs];
		for i in 0..rs {
			for j in 0..cs {
				if grid[i][j] == 1 { // thieves
					v.push((i,j));
				}
			}
		}
		// let mut visited = vec![vec![false; cs as usize]; rs as usize];
		// let mut stack = vec![];
		for (r, c) in v { // bfs make
			Self::mark_for_thief(&mut safe_map, r as _, c as _, rs , cs);
		}
		let factor_max = *safe_map.iter().flatten().max().unwrap();
		// println!("{:?}", safe_map);
		// println!("factor_max: {}", factor_max);
		(safe_map, factor_max)
	}
	fn mark_for_thief(safe_map: &mut Vec<Vec<i32>>, r: i32, c: i32, rs: usize, cs: usize) {
		let mut visited = vec![vec![false; cs as usize]; rs as usize];
		let mut stack = VecDeque::<(i32, i32, i32)>::new();
		stack.push_back((r, c, 0));
		while !stack.is_empty() {
			let (i, j, depth) = stack.pop_front().unwrap();
			safe_map[i as usize][j as usize] = depth.min(safe_map[i as usize][j as usize]);
			visited[i as usize][j as usize] = true;
			for (x, y) in [(i-1, j), (i+1, j), (i, j-1), (i, j+1)] {
				if x < 0 || x >= rs as i32 || y < 0 || y >= cs as i32 {
					continue;
				}
				if visited[x as usize][y as usize] {
					continue;
				}
				stack.push_back((x, y, depth+1));
			}
		}
	}
}

use std::collections::{BinaryHeap};
use std::fmt::Debug;

/// A 2D grid that provides methods to access elements in the grid. The indexing
/// type is flexible and can be any type that can be converted to `usize`.
///
pub struct Grid2D<T>(Vec<Vec<T>>);

impl<T: Copy> Grid2D<T> {
	/// Creates a new instance of the `Grid2D` struct.
	///
	pub fn new(g: Vec<Vec<T>>) -> Self {
		Self(g)
	}

	/// Returns the number of rows in the grid.
	///
	pub fn n_rows(&self) -> usize {
		self.0.len()
	}

	/// Returns the number of columns in the grid.
	///
	pub fn n_cols<I>(&self, row: I) -> Option<usize>
		where
			I: TryInto<usize>,
			<I as TryInto<usize>>::Error: Debug,
	{
		if let Ok(row) = row.try_into() {
			self.0.get(row).map(|r| r.len())
		} else {
			None
		}
	}

	/// Returns a reference to the element at the specified row and column.
	///
	pub fn get<I>(&self, i: I, j: I) -> Option<&T>
		where
			I: TryInto<usize>,
			<I as TryInto<usize>>::Error: Debug,
	{
		if let (Ok(i), Ok(j)) = (i.try_into(), j.try_into()) {
			self.0.get(i).and_then(|v| v.get(j))
		} else {
			None
		}
	}

	/// Returns a mutable reference to the element at the specified row and
	///
	pub fn get_mut<I>(&mut self, i: I, j: I) -> Option<&mut T>
		where
			I: TryInto<usize>,
			<I as TryInto<usize>>::Error: Debug,
	{
		if let (Ok(i), Ok(j)) = (i.try_into(), j.try_into()) {
			self.0.get_mut(i).and_then(|v| v.get_mut(j))
		} else {
			None
		}
	}
}

const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

struct Solution1;

impl Solution1 {
	pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
		let n = grid.len();
		let mut grid  = Grid2D::new(grid);
		let mut seen  = Grid2D::new(vec![vec![false; n]; n]);
		let mut deque = VecDeque::new();

		for i in 0..n as isize {
			for j in 0..n as isize {

				if grid.get(i, j) == Some(&1) {

					deque.push_back((i, j, 2));

					while let Some((i, j, d)) = deque.pop_front() {
						for &(di, dj) in &DIRS {
							let (i, j) = (i + di, j + dj);

							if let Some(&v) = grid.get(i, j) {
								if v == 0 || v > d {
									*grid.get_mut(i, j).unwrap() = d;
									deque.push_back((i, j, d + 1));
								}
							}
						}
					}
				}
			}
		}
		drop(deque);
		let mut heap  = BinaryHeap::new();
		let mut dists = Grid2D::new(vec![vec![0; n]; n]);

		heap.push((*grid.get(0, 0).unwrap(), 0, 0));

		while let Some((dist1, i, j)) = heap.pop() {
			*seen.get_mut(i, j).unwrap() = true;

			for &(di, dj) in &DIRS {
				let (i, j) = (i + di, j + dj);

				if seen.get(i, j).map_or(true, |&v| v) {
					continue;
				}
				let dist2 = grid.get(i, j).map_or(0, |&v| v).min(dist1);

				if dists.get(i, j).map_or(false, |&v| v < dist2) {
					*dists.get_mut(i, j).unwrap() = dist2;
					heap.push((dist2, i, j));
				}
			}
		}
		dists.get(n - 1, n - 1).map_or(0, |d| (d - 1).max(0))
	}
}

struct Solution2;

// use std::collections::{BinaryHeap, VecDeque};

impl Solution2 {
	const NEIGHBORS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

	pub fn maximum_safeness_factor(mut grid: Vec<Vec<i32>>) -> i32 {
		let n = grid.len();
		let valid_neighbors = |(i, j): (usize, usize)| {
			let (n, r, c) = (n as i32, i as i32, j as i32);
			Self::NEIGHBORS
				.iter()
				.map(move |(dr, dc)| (r + dr, c + dc))
				.filter(move |&(x, y)| 0 <= x && x < n && 0 <= y && y < n)
				.map(|(x, y)| (x as usize, y as usize))
		};

		let mut queue: VecDeque<_> = (0..n)
			.flat_map(|i| (0..n).map(move |j| (i, j)))
			.filter_map(|(i, j)| match grid[i][j] {
				0 => {
					grid[i][j] = -1;
					None
				}
				_ => {
					grid[i][j] = 0;
					Some((i, j))
				}
			})
			.collect();
		let mut alt = queue.clone();

		while !queue.is_empty() {
			alt.clear();
			while let Some((r, c)) = queue.pop_front() {
				alt.extend(valid_neighbors((r, c)).filter_map(|(x, y)| {
					if grid[x][y] != -1 {
						return None;
					}
					grid[x][y] = grid[r][c] + 1;
					Some((x, y))
				}));
			}
			std::mem::swap(&mut queue, &mut alt)
		}

		// the biggest factor of safety will be on top of the heap
		//
		let mut heap = BinaryHeap::from([(grid[0][0], 0, 0)]);
		while let Some((safety, i, j)) = heap.pop() {
			if i == n - 1 && j == n - 1 {
				return safety;
			}
			heap.extend(valid_neighbors((i, j)).filter_map(|(x, y)| {
				let val = grid[x][y];
				if val == -1 {
					return None;
				}
				grid[x][y] = -1;
				Some((safety.min(val), x, y))
			}));
		}

		0
	}
}

fn print_matrix(matrix: &Vec<Vec<i32>>) {
	for row in matrix {
		println!("{row:?}");
	}
	println!("");
}

fn main() {
    println!("Hello, world!");
}
