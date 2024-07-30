

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		// [[0,0,1,1],[1,0,1,0],[1,1,0,0]]
		let grid = vec![
			vec![0,0,1,1],
			vec![1,0,1,0],
			vec![1,1,0,0]
		];
		let result = Solution::matrix_score(grid);
		println!("{:?}", result);
	}

	#[test]
	fn test2() {
		// [[0]]
		let grid = vec![
			vec![0]
		];
		let result = Solution::matrix_score(grid);
		println!("{:?}", result);
	}


	#[test]
	fn test11() {
		let grid = vec![
			vec![0,0,1,1],
			vec![1,0,1,0],
			vec![1,1,0,0]
		];
		let result = Solution1::matrix_score(grid);
		println!("{:?}", result);
	}

	#[test]
	fn test21() {
		// [[0]]
		let grid = vec![
			vec![0]
		];
		let result = Solution1::matrix_score(grid);
		println!("{:?}", result);
	}

	#[test]
	fn test12() {
		let grid = vec![
			vec![0,0,1,1],
			vec![1,0,1,0],
			vec![1,1,0,0]
		];
		let result = Solution2::matrix_score(grid);
		println!("{:?}", result);
	}

	#[test]
	fn test22() {
		// [[0]]
		let grid = vec![
			vec![0]
		];
		let result = Solution2::matrix_score(grid);
		println!("{:?}", result);
	}

}






