

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(nums: Vec<i32>, k: i32, expected: i32) {
		let result = Solution::number_of_subarrays(nums, k);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let nums = vec![1, 1, 2, 1, 1];
		let k = 3;
		t(nums, k, 2);
	}

	#[test]
	fn test2() {
		let nums = vec![2, 4, 6];
		let k = 1;
		t(nums, k, 0);
	}

	#[test]
	fn test3() {
		let nums = vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2];
		let k = 2;
		t(nums, k, 16);
	}

}