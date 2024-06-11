

#[cfg(test)]
mod tests {
	use crate::*;

	fn t(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>, expected: i64) {
		let result = Solution::largest_square_area(bottom_left, top_right);
		println!("{}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		// bottomLeft = [[1,1],[2,2],[3,1]], topRight = [[3,3],[4,4],[6,6]]
		let left = vec![vec![1,1], vec![2,2], vec![3,1]];
		let right = vec![vec![3,3], vec![4,4], vec![6,6]];
		let expected = 1;
		t(left, right, expected);
	}

	#[test]
	fn test2() {
		// bottomLeft = [[1,1],[2,2],[1,2]], topRight = [[3,3],[4,4],[3,4]]
		let left = vec![vec![1,1], vec![2,2], vec![1,2]];
		let right = vec![vec![3,3], vec![4,4], vec![3,4]];
		let expected = 1;
		t(left, right, expected);
	}

	#[test]
	fn test3() {
		// bottomLeft = [[1,1],[3,3],[3,1]], topRight = [[2,2],[4,4],[4,2]]
		let left = vec![vec![1,1], vec![3,3], vec![3,1]];
		let right = vec![vec![2,2], vec![4,4], vec![4,2]];
		let expected = 0;
		t(left, right, expected);
	}

}