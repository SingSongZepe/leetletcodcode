

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(nums: Vec<i32>, expected: i32) {
		let result = Solution::min_difference(nums);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t1(nums: Vec<i32>, expected: i32) {
		let result = Solution1::min_difference(nums);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let nums = vec![5, 3, 2, 4, 1];
		t(nums, 1);
	}

	#[test]
	fn test2() {
		let nums = vec![1, 5, 0, 10, 14];
		t(nums, 1);
	}

	#[test]
	fn test3() {
		let nums = vec![3, 100, 20];
		t(nums, 0);
	}

	#[test]
	fn test4() {
		let nums = vec![6,6,0,1,1,4,6];
		t(nums, 2);
	}

	#[test]
	fn test11() {
		let nums = vec![5, 3, 2, 4, 1];
		t1(nums, 1);
	}

	#[test]
	fn test21() {
		let nums = vec![1, 5, 0, 10, 14];
		t1(nums, 1);
	}

	#[test]
	fn test31() {
		let nums = vec![3, 100, 20];
		t1(nums, 0);
	}
}