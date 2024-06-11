

#[cfg(test)]
mod tests {
	use crate::*;

	fn t(nums: Vec<i32>, expected: Vec<i32>) {
		let result = Solution::number_of_pairs(nums);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let nums = vec![1,3,2,1,3,2,2];
		t(nums, vec![3, 1])
	}

	#[test]
	fn test2() {
		let nums = vec![1, 1];
		t(nums, vec![1, 0])
	}

	#[test]
	fn test3() {
		let nums = vec![0];
		t(nums, vec![0, 1])
	}
}