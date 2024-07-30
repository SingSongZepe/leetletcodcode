

#[cfg(test)]
mod tests {
	use crate::*;

	fn t(nums: Vec<i32>, expected: i32) {
		let result = Solution::minimum_right_shifts(nums);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	#[test]
	fn test1() {
		let nums = vec![3, 4, 5, 1, 2];
		t(nums, 2);
	}

	#[test]
	fn test2() {
		let nums = vec![1, 2, 3];
		t(nums, 0);
	}

	#[test]
	fn test3() {
		let nums = vec![2, 1, 4];
		t(nums, -1);
	}

	#[test]
	fn test4() {
		let nums = vec![0];
		t(nums, 0);
	}
}