

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(nums: Vec<i32>, expected: f64) {
		let result = Solution::minimum_average(nums);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let nums = vec![7,8,3,4,15,13,4,1];
		t(nums, 5.5);
	}


	#[test]
	fn test2() {
		let nums = vec![1, 9, 8, 3, 10, 5];
		t(nums, 5.5);
	}

	#[test]
	fn test3() {
		let nums = vec![1,2,3,7,8,9];
		t(nums, 5.0);
	}


}