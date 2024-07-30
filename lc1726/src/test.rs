

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(nums: Vec<i32>, expected: i32) {
		let result = Solution::tuple_same_product(nums);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let nums = vec![2, 3, 4, 6];
		t(nums, 8);
	}

	#[test]
	fn test2() {
		let nums = vec![1, 2, 4, 5, 10];
		t(nums, 16);
	}

}