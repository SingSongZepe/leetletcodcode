

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(nums: Vec<i32>, expected: i32) {
		let result = Solution::minimum_operations(nums);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let nums = vec![1, 2, 3, 4];
		let expected = 3;
		t(nums, expected);
	}

	#[test]
	fn test2() {
		let nums = vec![3, 6, 9];
		let expected = 0;
		t(nums, expected);
	}

}