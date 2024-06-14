mod test;

struct Solution;

use std::collections::HashSet;

struct PointUnion {
	count: i32,
	xset: HashSet<i32>,
	yset: HashSet<i32>,
}

impl PointUnion {
	fn new() -> Self {
		PointUnion {
			count: 0,
			xset: HashSet::new(),
			yset: HashSet::new(),
		}
	}
	fn new_with_point(x: i32, y: i32) -> Self {
		PointUnion {
			count: 1,
			xset: HashSet::from([x]),
			yset: HashSet::from([y]),
		}
	}
	fn x_axis_in(&self, x: i32) -> bool {
		if self.xset.contains(&x) {
			true
		} else {
			false
		}
	}
	fn y_axis_in(&self, y: i32) -> bool {
		if self.yset.contains(&y) {
			true
		} else {
			false
		}
	}
	fn add_point(&mut self, x: i32, y: i32) {
		self.count += 1;
		self.xset.insert(x);
		self.yset.insert(y);
	}
	fn check_and_join(&mut self, pu: &PointUnion) -> bool {
		let join_set = |s1: &mut HashSet<i32>, s2: &HashSet<i32>| {
			for &n in s2.iter() {
				s1.insert(n);
			}
		};
		for &x in pu.xset.iter() {
			if self.x_axis_in(x) {
				self.count += pu.count;
				join_set(&mut self.xset, &pu.xset);
				join_set(&mut self.yset, &pu.yset);
				return true;
			}
		}
		for &y in pu.yset.iter() {
			if self.y_axis_in(y) {
				self.count += pu.count;
				join_set(&mut self.xset, &pu.xset);
				join_set(&mut self.yset, &pu.yset);
				return true;
			}
		}
		false
	}
}

impl Clone for PointUnion {
	fn clone(&self) -> Self {
		PointUnion {
			count: self.count,
			xset: self.xset.clone(),
			yset: self.yset.clone(),
		}
	}
}


impl Solution {
	pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
		let mut pus: Vec<PointUnion> = vec![];
		let (mut xts, mut yts) = (HashSet::<i32>::new(), HashSet::<i32>::new());
		for st in stones {
			if !xts.contains(&st[0]) && !yts.contains(&st[1]) {
				let pu = PointUnion::new_with_point(st[0], st[1]);
				pus.push(pu);
			} else {
				for pu in pus.iter_mut() {
					if pu.x_axis_in(st[0]) || pu.y_axis_in(st[1]) {
						pu.add_point(st[0], st[1]);
						break;
					}
				}
			}
			xts.insert(st[0]);
			yts.insert(st[1]);
		}
		let mut rpus = vec![pus[0].clone()];

		'lp1:
		for pu in pus.iter_mut().skip(1) {
			for rpu in rpus.iter_mut() {
				if rpu.check_and_join(pu) {
					continue 'lp1;
				}
			}
			rpus.push(pu.clone());
		}
		rpus.iter().fold(0, |acc, pu| acc + pu.count) - rpus.len() as i32
	}
}

// solution 1

use std::collections::{BTreeSet, HashMap};
use std::convert::{TryFrom, TryInto};

struct Solution1;

impl Solution1 {
	pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
		// Use better format of input data.
		let stones: Vec<(i32, i32)> = stones
			.into_iter()
			.map(|coord| {
				assert_eq!(coord.len(), 2);
				(coord[0], coord[1])
			})
			.collect();

		let mut row_to_first_idx = HashMap::with_capacity(stones.len());
		let mut col_to_first_idx = HashMap::with_capacity(stones.len());
		for (i, &(row, col)) in stones.iter().enumerate() {
			let i = u32::try_from(i).unwrap();
			row_to_first_idx.entry(row).or_insert(i);
			col_to_first_idx.entry(col).or_insert(i);
		}

		let n: u32 = stones.len().try_into().unwrap();
		let mut djs = DisjointSet::new(n);
		for (i, &(row, col)) in stones.iter().enumerate() {
			let i = u32::try_from(i).unwrap();
			let first_row_idx = *row_to_first_idx.get(&row).unwrap();
			let first_col_idx = *col_to_first_idx.get(&col).unwrap();
			djs.union(first_row_idx, i);
			djs.union(first_col_idx, i);
		}
		// Get each unique group.
		let unique_roots: BTreeSet<u32> = (0..n).map(|i| djs.get_root(i)).collect();
		// We keep 1 stone for each connected group.
		let remaining_stones = unique_roots.len();
		let existed_stones = stones.len();
		(existed_stones - remaining_stones).try_into().unwrap()
	}
}

struct DisjointSet {
	// Use one buffer for both roots and ranks
	// to minimize allocations.
	// See `fn roots_and_ranks`.
	data: Box<[u32]>,
}

#[inline]
#[track_caller]
fn u2s(idx: u32) -> usize {
	idx.try_into().unwrap()
}

impl DisjointSet {
	fn new(size: u32) -> Self {
		let double_size = usize::try_from(size).unwrap() * 2;
		let mut res = Self {
			data: vec![0; double_size].into(),
		};
		let (roots, ranks) = res.roots_and_ranks();
		for (i, v) in roots.iter_mut().enumerate() {
			*v = u32::try_from(i).unwrap();
		}
		ranks.fill(1);
		res
	}

	fn roots_and_ranks(&mut self) -> (&mut [u32], &mut [u32]) {
		let half_len = self.data.len() / 2;
		self.data.split_at_mut(half_len)
	}

	fn get_root(&mut self, position: u32) -> u32 {
		let (roots, _) = self.roots_and_ranks();
		let mut current = position;
		while roots[u2s(current)] != current {
			current = roots[u2s(current)];
		}
		let root = current;
		if roots[u2s(position)] == root {
			return root;
		}

		current = position;
		while current != root {
			let nxt = roots[u2s(current)];
			roots[u2s(current)] = root;
			current = nxt;
		}
		root
	}

	fn union(&mut self, f: u32, s: u32) {
		use std::cmp::Ordering;

		if f == s {
			return;
		}

		let root_f = self.get_root(f);
		let root_s = self.get_root(s);
		if root_f == root_s {
			return;
		}

		let (roots, ranks) = self.roots_and_ranks();
		let rank_f = ranks[u2s(root_f)];
		let rank_s = ranks[u2s(root_s)];
		match rank_f.cmp(&rank_s) {
			Ordering::Less => roots[u2s(root_f)] = root_s,
			Ordering::Greater => roots[u2s(root_s)] = root_f,
			Ordering::Equal => {
				let new_rank = rank_f + 1;
				roots[u2s(root_s)] = root_f;
				ranks[u2s(root_f)] = new_rank;
			}
		}
	}
}

fn main() {
    println!("Hello, world!");
}
