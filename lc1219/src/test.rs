

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		// [[0,6,0],[5,8,7],[0,9,0]]
		let grid = vec![
			vec![0,6,0],
			vec![5,8,7],
			vec![0,9,0],
		];
		let result = Solution::get_maximum_gold(grid);
		println!("{}", result); // expected output: 24
	}

	#[test]
	fn test2() {
		// [[1,0,7],[2,0,6],[3,4,5],[0,3,0],[9,0,20]]
		let grid = vec![
			vec![1,0,7],
			vec![2,0,6],
			vec![3,4,5],
			vec![0,3,0],
			vec![9,0,20],
		];
		let result = Solution::get_maximum_gold(grid);
		println!("{}", result); // expected output: 28
	}

	#[test]
	fn test11() {
		// [[0,6,0],[5,8,7],[0,9,0]]
		let grid = vec![
			vec![0,6,0],
			vec![5,8,7],
			vec![0,9,0],
		];
		let result = Solution1::get_maximum_gold(grid);
		println!("{}", result); // expected output: 24
	}

	#[test]
	fn test21() {
		// [[1,0,7],[2,0,6],[3,4,5],[0,3,0],[9,0,20]]
		let grid = vec![
			vec![1,0,7],
			vec![2,0,6],
			vec![3,4,5],
			vec![0,3,0],
			vec![9,0,20],
		];
		let result = Solution1::get_maximum_gold(grid);
		println!("{}", result); // expected output: 28
	}


	// solution 2
	#[test]
	fn test12() {
		// [[0,6,0],[5,8,7],[0,9,0]]
		let grid = vec![
			vec![0,6,0],
			vec![5,8,7],
			vec![0,9,0],
		];
		let result = Solution2::get_maximum_gold(grid);
		println!("{}", result); // expected output: 24
	}

	#[test]
	fn test22() {
		// [[1,0,7],[2,0,6],[3,4,5],[0,3,0],[9,0,20]]
		let grid = vec![
			vec![1,0,7],
			vec![2,0,6],
			vec![3,4,5],
			vec![0,3,0],
			vec![9,0,20],
		];
		let result = Solution2::get_maximum_gold(grid);
		println!("{}", result); // expected output: 28
	}
}