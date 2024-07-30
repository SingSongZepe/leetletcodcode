

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		//  [[1,0,1],[0,0,0],[1,0,1]]
		let grid = vec![
			vec![1,0,1],
			vec![0,0,0],
			vec![1,0,1],
		];
		let result = Solution::max_distance(grid);
		println!("{}", result);
	}

	#[test]
	fn test2() {
		//  [1,0,0],[0,0,0],[0,0,0]]
		let grid = vec![
			vec![1,0,0],
			vec![0,0,0],
			vec![0,0,0],
		];
		let result = Solution::max_distance(grid);
		println!("{}", result);
	}

	#[test]
	fn test3() {
		//  [[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0]]
		let grid = vec![
			vec![0,0,0,0],
			vec![0,0,0,0],
			vec![0,0,0,0],
			vec![0,0,0,0],
		];
		let result = Solution::max_distance(grid);
		println!("{}", result);
	}

	#[test]
	fn test4() {
		//  [[1,1,1,1,1],[1,1,1,1,1],[1,1,1,1,1],[1,1,1,1,1],[1,1,1,1,1]]
		let grid = vec![
			vec![1,1,1,1,1],
			vec![1,1,1,1,1],
			vec![1,1,1,1,1],
			vec![1,1,1,1,1],
			vec![1,1,1,1,1],
		];
		let result = Solution::max_distance(grid);
		println!("{}", result);
	}

	// solution
	#[test]
	fn test11() {
		//  [[1,0,1],[0,0,0],[1,0,1]]
		let grid = vec![
			vec![1,0,1],
			vec![0,0,0],
			vec![1,0,1],
		];
		let result = Solution1::max_distance(grid);
		println!("{}", result);
	}

	#[test]
	fn test21() {
		//  [1,0,0],[0,0,0],[0,0,0]]
		let grid = vec![
			vec![1,0,0],
			vec![0,0,0],
			vec![0,0,0],
		];
		let result = Solution1::max_distance(grid);
		println!("{}", result);
	}

	#[test]
	fn test31() {
		//  [[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0]]
		let grid = vec![
			vec![0,0,0,0],
			vec![0,0,0,0],
			vec![0,0,0,0],
			vec![0,0,0,0],
		];
		let result = Solution1::max_distance(grid);
		println!("{}", result);
	}

	#[test]
	fn test41() {
		//  [[1,1,1,1,1],[1,1,1,1,1],[1,1,1,1,1],[1,1,1,1,1],[1,1,1,1,1]]
		let grid = vec![
			vec![1,1,1,1,1],
			vec![1,1,1,1,1],
			vec![1,1,1,1,1],
			vec![1,1,1,1,1],
			vec![1,1,1,1,1],
		];
		let result = Solution1::max_distance(grid);
		println!("{}", result);
	}
}