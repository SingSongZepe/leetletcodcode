

#[cfg(test)]
mod tests {
	use crate::*;

	fn t(nums: Vec<i32>, expected: i32) {
		let result = Solution::minimum_operations(nums);
		println!("{}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let nums = vec![1, 5, 0, 3, 5];
		t(nums, 3)
	}

	#[test]
	fn test2() {
		let nums = vec![0];
		t(nums, 0)
	}

}