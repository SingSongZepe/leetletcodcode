

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		// [[9,9,8,1],[5,6,2,6],[8,2,6,4],[6,2,2,2]]
		let grid = vec![
			vec![9,9,8,1],
			vec![5,6,2,6],
			vec![8,2,6,4],
			vec![6,2,2,2]
		];
		let result = Solution::largest_local(grid);
		print_matrix(&result);
	}

	#[test]
	fn test2() {
		// [[9,9,8,1],[5,6,2,6],[8,2,6,4],[6,2,2,2]]
		let grid = vec![
			vec![9,9,8,1],
			vec![5,6,2,6],
			vec![8,2,6,4],
			vec![6,2,2,2]
		];
		let result = Solution1::largest_local(grid);
		print_matrix(&result);
	}

	#[test]
	fn test3() {
		// [[9,9,8,1],[5,6,2,6],[8,2,6,4],[6,2,2,2]]
		let grid = vec![
			vec![9,9,8,1],
			vec![5,6,2,6],
			vec![8,2,6,4],
			vec![6,2,2,2]
		];
		let result = Solution2::largest_local(grid);
		print_matrix(&result);
	}

}