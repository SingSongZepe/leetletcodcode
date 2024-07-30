

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(arr: Vec<i32>, expected: bool) {
		let result = Solution::three_consecutive_odds(arr);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t1(arr: Vec<i32>, expected: bool) {
		let result = Solution1::three_consecutive_odds(arr);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}


	#[test]
	fn test1() {
		let arr = vec![2, 6, 4, 1];
		let expected = false;
		t(arr, expected);
	}

	#[test]
	fn test2() {
		let arr = vec![1, 2, 34, 3, 4, 5, 7, 23, 12];
		let expected = true;
		t(arr, expected);
	}

	#[test]
	fn test11() {
		let arr = vec![2, 6, 4, 1];
		let expected = false;
		t1(arr, expected);
	}

	#[test]
	fn test21() {
		let arr = vec![1, 2, 34, 3, 4, 5, 7, 23, 12];
		let expected = true;
		t1(arr, expected);
	}

	#[test]
	fn test31() {
		let arr = vec![1, 1, 1];
		let expected = true;
		t1(arr, expected);
	}
}