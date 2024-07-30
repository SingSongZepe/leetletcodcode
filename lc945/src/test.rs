

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(arg: Vec<i32>, expected: i32) {
		let result = Solution::min_increment_for_unique(arg);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let nums = vec![1, 2, 2];
		t(nums, 1)
	}

	#[test]
	fn test2() {
		let nums = vec![3, 2, 1, 2, 1, 7];
		t(nums, 6)
	}
}