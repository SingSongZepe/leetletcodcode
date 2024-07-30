

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(nums: Vec<i32>, expected: i32) {
		let result = Solution::missing_number(nums);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t1(nums: Vec<i32>, expected: i32) {
		let result = Solution1::missing_number(nums);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let nums = vec![3, 0, 1];
		t(nums, 2);
	}

	#[test]
	fn test2() {
		let nums = vec![0, 1];
		t(nums, 2);
	}

	#[test]
	fn test3() {
		let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
		t(nums, 8);
	}

	#[test]
	fn test11() {
		let nums = vec![3, 0, 1];
		t1(nums, 2);
	}

	#[test]
	fn test21() {
		let nums = vec![0, 1];
		t1(nums, 2);
	}

	#[test]
	fn test31() {
		let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
		t1(nums, 8);
	}
}