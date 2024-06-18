

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(rectangles: Vec<Vec<i32>>, expected: i32) {
		let result = Solution::count_good_rectangles(rectangles);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let rectangles = vec![vec![5,8], vec![3,9], vec![5,12], vec![16,5]];
		t(rectangles, 3);
	}

	#[test]
	fn test2() {
		let rectangles = vec![vec![2, 3], vec![3, 7], vec![4, 3], vec![3, 7]];
		t(rectangles, 3);
	}

	#[test]
	fn test_count_max_value() {
		let r = vec![4, 4, 4, 4, 3];
	}

}