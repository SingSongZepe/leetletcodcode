

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(matrix: Vec<Vec<i32>>, expected: bool) {
		let result = Solution::is_toeplitz_matrix(matrix);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t1(matrix: Vec<Vec<i32>>, expected: bool) {
		let result = Solution1::is_toeplitz_matrix(matrix);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		// matrix = [[1,2,3,4],[5,1,2,3],[9,5,1,2]]
		let matrix = vec![vec![1,2,3,4], vec![5,1,2,3], vec![9,5,1,2]];
		t(matrix, true);
	}

	#[test]
	fn test2() {
		// matrix = [[1,2],[2,2]]
		let matrix = vec![vec![1,2], vec![2,2]];
		t(matrix, false);
	}

	#[test]
	fn test3() {
		// matrix = [[18],[66]]
		let matrix = vec![vec![18], vec![66]];
		t(matrix, true);
	}

	#[test]
	fn test4() {
		// matrix = [[11,74,0,93],[40,11,74,7]]
		let matrix = vec![vec![11,74,0,93], vec![40,11,74,7]];
		t(matrix, false);
	}

	#[test]
	fn test5() {
		// matrix = [[53,64,90,98,34],[91,53,64,90,98],[17,91,53,64,0]]
		let matrix = vec![vec![53,64,90,98,34], vec![91,53,64,90,98], vec![17,91,53,64,0]];
		t(matrix, false);
	}



	#[test]
	fn test11() {
		// matrix = [[1,2,3,4],[5,1,2,3],[9,5,1,2]]
		let matrix = vec![vec![1,2,3,4], vec![5,1,2,3], vec![9,5,1,2]];
		t1(matrix, true);
	}

	#[test]
	fn test21() {
		// matrix = [[1,2],[2,2]]
		let matrix = vec![vec![1,2], vec![2,2]];
		t1(matrix, false);
	}

	#[test]
	fn test31() {
		// matrix = [[18],[66]]
		let matrix = vec![vec![18], vec![66]];
		t1(matrix, true);
	}

	#[test]
	fn test41() {
		// matrix = [[11,74,0,93],[40,11,74,7]]
		let matrix = vec![vec![11,74,0,93], vec![40,11,74,7]];
		t1(matrix, false);
	}

	#[test]
	fn test51() {
		// matrix = [[53,64,90,98,34],[91,53,64,90,98],[17,91,53,64,0]]
		let matrix = vec![vec![53,64,90,98,34], vec![91,53,64,90,98], vec![17,91,53,64,0]];
		t1(matrix, false);
	}
}