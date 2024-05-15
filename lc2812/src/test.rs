

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		//[[1,0,0],[0,0,0],[0,0,1]]
		let grid = vec![
			vec![1,0,0],
			vec![0,0,0],
			vec![0,0,1]
		];
		let result = Solution::maximum_safeness_factor(grid);
		println!("{}", result);
	}

	#[test]
	fn test2() {
		//[[0,0,1],[0,0,0],[0,0,0]]
		let grid = vec![
			vec![0,0,1],
			vec![0,0,0],
			vec![0,0,0]
		];
		let result = Solution::maximum_safeness_factor(grid);
		println!("{}", result);
	}

	#[test]
	fn test3() {
		//[[0,0,0,1],[0,0,0,0],[0,0,0,0],[1,0,0,0]]
		let grid = vec![
			vec![0,0,0,1],
			vec![0,0,0,0],
			vec![0,0,0,0],
			vec![1,0,0,0]
		];
		let result = Solution::maximum_safeness_factor(grid);
		println!("{}", result);
	}

	#[test]
	fn test4() {
		//[[0,0,1],[0,0,1],[0,0,0]]
		let grid = vec![
			vec![0,0,1],
			vec![0,0,1],
			vec![0,0,0]
		];
		let result = Solution::maximum_safeness_factor(grid);
		println!("{}", result);
	}

	#[test]
	fn test5() {
		//[[0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1],[0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1],[0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1],[0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1],[1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1],[1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],[1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0],[1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0],[1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0],[1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0],[1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0],[1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0],[1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0]]
		let grid = vec![
			vec![0,0,0,0,0,1,1,1,1,1,1,1,1],
			vec![0,0,0,0,0,0,1,1,1,1,1,1,1],
			vec![0,0,0,0,0,0,0,1,1,1,1,1,1],
			vec![0,0,0,0,0,0,0,0,1,1,1,1,1],
			vec![0,0,0,0,0,0,0,0,0,1,1,1,1],
			vec![0,0,0,0,0,0,0,0,0,1,1,1,1],
			vec![0,0,0,0,0,0,0,0,0,1,1,1,1],
			vec![1,1,0,0,0,0,0,0,0,0,0,1,1],
			vec![1,1,1,0,0,0,0,0,0,0,0,0,1],
			vec![1,1,1,1,0,0,0,0,0,0,0,0,0],
			vec![1,1,1,1,1,0,0,0,0,0,0,0,0],
		];
		let result = Solution::maximum_safeness_factor(grid);
		println!("{}", result);
	}

	// solution 2
	#[test]
	fn test12() {
		//[[1,0,0],[0,0,0],[0,0,1]]
		let grid = vec![
			vec![1,0,0],
			vec![0,0,0],
			vec![0,0,1]
		];
		let result = Solution2::maximum_safeness_factor(grid);
		println!("{}", result);
	}

	#[test]
	fn test22() {
		//[[0,0,1],[0,0,0],[0,0,0]]
		let grid = vec![
			vec![0,0,1],
			vec![0,0,0],
			vec![0,0,0]
		];
		let result = Solution2::maximum_safeness_factor(grid);
		println!("{}", result);
	}

	#[test]
	fn test32() {
		//[[0,0,0,1],[0,0,0,0],[0,0,0,0],[1,0,0,0]]
		let grid = vec![
			vec![0,0,0,1],
			vec![0,0,0,0],
			vec![0,0,0,0],
			vec![1,0,0,0]
		];
		let result = Solution2::maximum_safeness_factor(grid);
		println!("{}", result);
	}

	#[test]
	fn test42() {
		//[[0,0,1],[0,0,1],[0,0,0]]
		let grid = vec![
			vec![0,0,1],
			vec![0,0,1],
			vec![0,0,0]
		];
		let result = Solution2::maximum_safeness_factor(grid);
		println!("{}", result);
	}

	#[test]
	fn test52() {
		//[[0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1],[0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1],[0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1],[0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1],[1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1],[1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],[1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0],[1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0],[1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0],[1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0],[1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0],[1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0],[1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0]]
		let grid = vec![
			vec![0,0,0,0,0,1,1,1,1,1,1,1,1],
			vec![0,0,0,0,0,0,1,1,1,1,1,1,1],
			vec![0,0,0,0,0,0,0,1,1,1,1,1,1],
			vec![0,0,0,0,0,0,0,0,1,1,1,1,1],
			vec![0,0,0,0,0,0,0,0,0,1,1,1,1],
			vec![0,0,0,0,0,0,0,0,0,1,1,1,1],
			vec![0,0,0,0,0,0,0,0,0,1,1,1,1],
			vec![1,1,0,0,0,0,0,0,0,0,0,1,1],
			vec![1,1,1,0,0,0,0,0,0,0,0,0,1],
			vec![1,1,1,1,0,0,0,0,0,0,0,0,0],
			vec![1,1,1,1,1,0,0,0,0,0,0,0,0],
		];
		let result = Solution2::maximum_safeness_factor(grid);
		println!("{}", result);
	}

	#[test]
	fn test_connect_by_factor() {
		let safe_grid = vec![
			vec![2,1,0],
			vec![1,2,1],
			vec![0,1,2]
		];
		let factor = 3;
		let rs = safe_grid.len();
		let cs = safe_grid[0].len();
		let result = Solution::connect_by_factor(&safe_grid, factor, rs, cs);
		println!("{}", result);
	}

	#[test]
	fn test_heap_with_vector() {
		let mut v1 = vec![1,2,3,4,5];
		let mut v2 = vec![6,7,8,9,10];
		let mut v3 = vec![11,12,13,14,15];

		let mut heap = BinaryHeap::new();
		heap.push(v3);
		heap.push(v1);
		heap.push(v2);

		while let Some(mut top) = heap.pop() {
			println!("{:?}", top);
		}
	}

	#[test]
	fn test_heap_with_tuple() {
		let mut v1 = (1,2,3,4,5);
		let mut v2 = (6,7,8,9,10);
		let mut v3 = (11,12,13,14,15);

		let mut heap = BinaryHeap::new();
		heap.push(v3);
		heap.push(v1);
		heap.push(v2);

		while let Some(mut top) = heap.pop() {
			println!("{:?}", top);
		}
	}
}