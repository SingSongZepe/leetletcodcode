

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(nums: Vec<i32>, limit: i32, expected: i32) {
		let result = Solution::longest_subarray(nums, limit);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t1(nums: Vec<i32>, limit: i32, expected: i32) {
		let result = Solution1::longest_subarray(nums, limit);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let nums = vec![8, 2, 4, 7];
		let limit = 4;
		let expected = 2;
		t(nums, limit, expected);
	}

	#[test]
	fn test2() {
		let nums = vec![10, 1, 2, 4, 7, 2];
		let limit = 5;
		let expected = 4;
		t(nums, limit, expected);
	}

	#[test]
	fn test3() {
		let nums = vec![4, 2, 2, 2, 4, 4, 2, 2];
		let limit = 0;
		let expected = 3;
		t(nums, limit, expected);
	}

	#[test]
	fn test11() {
		let nums = vec![8, 2, 4, 7];
		let limit = 4;
		let expected = 2;
		t(nums, limit, expected);
	}

	#[test]
	fn test21() {
		let nums = vec![10, 1, 2, 4, 7, 2];
		let limit = 5;
		let expected = 4;
		t(nums, limit, expected);
	}

	#[test]
	fn test31() {
		let nums = vec![4, 2, 2, 2, 4, 4, 2, 2];
		let limit = 0;
		let expected = 3;
		t(nums, limit, expected);
	}
}