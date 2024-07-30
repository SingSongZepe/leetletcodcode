

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(arg: Vec<i32>, expected: i32) {
		let result = Solution::sum_of_squares(arg);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let nums = vec![1, 2, 3, 4];
		t(nums, 21);
	}

	#[test]
	fn test2() {
		let nums = vec![2,7,1,19,18,3];
		t(nums, 63);
	}
}