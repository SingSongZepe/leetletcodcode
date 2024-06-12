

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(arg: Vec<i32>, expected: i64) {
		let result = Solution::maximum_triplet_value(arg);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let nums = vec![12, 6, 1, 2, 7];
		t(nums, 77);
	}

	#[test]
	fn test2() {
		let nums = vec![1, 10, 3, 4, 19];
		t(nums, 133);
	}

	#[test]
	fn test3() {
		let nums = vec![1, 2, 3];
		t(nums, 0);
	}
}