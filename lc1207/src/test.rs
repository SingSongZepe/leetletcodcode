

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(arr: Vec<i32>, expected: bool) {
		let result = Solution::unique_occurrences(arr);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t1(arr: Vec<i32>, expected: bool) {
		let result = Solution1::unique_occurrences(arr);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let arr = vec![1,2,2,1,1,3];
		t(arr, true)
	}

	#[test]
	fn test2() {
		let arr = vec![1,2];
		t(arr, false)
	}

	#[test]
	fn test3() {
		let arr = vec![-3,0,1,-3,1,1,1,-3,10,0];
		t(arr, true);
	}

	#[test]
	fn test11() {
		let arr = vec![1,2,2,1,1,3];
		t1(arr, true)
	}

	#[test]
	fn test21() {
		let arr = vec![1,2];
		t1(arr, false)
	}

	#[test]
	fn test31() {
		let arr = vec![-3,0,1,-3,1,1,1,-3,10,0];
		t1(arr, true);
	}
}