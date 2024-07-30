

#[cfg(test)]
mod tests {
	use crate::*;

	fn t(nums: Vec<i32>, change_indices: Vec<i32>, expected: i32) {
		let result = Solution::earliest_second_to_mark_indices(nums, change_indices);
		println!("{}", result);
		assert_eq!(result, expected);
	}

	fn t1(nums: Vec<i32>, change_indices: Vec<i32>, expected: i32) {
		let result = Solution1::earliest_second_to_mark_indices(nums, change_indices);
		println!("{}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		// [2,2,0], changeIndices = [2,2,2,2,3,2,2,1]
		let nums = vec![2,2,0];
		let change_indices = vec![2,2,2,2,3,2,2,1];
		let expected = 8;
		t(nums, change_indices, expected);
	}

	#[test]
	fn test2() {
		// nums = [1,3], changeIndices = [1,1,1,2,1,1,1]
		let nums = vec![1,3];
		let change_indices = vec![1,1,1,2,1,1,1];
		let expected = 6;
		t(nums, change_indices, expected);
	}

	#[test]
	fn test3() {
		// nums = [0,1], changeIndices = [2,2,2]
		let nums = vec![0, 1];
		let change_indices = vec![2, 2, 2];
		let expected = -1;
		t(nums, change_indices, expected);
	}

	#[test]
	fn test11() {
		// [2,2,0], changeIndices = [2,2,2,2,3,2,2,1]
		let nums = vec![2,2,0];
		let change_indices = vec![2,2,2,2,3,2,2,1];
		let expected = 8;
		t1(nums, change_indices, expected);
	}

	#[test]
	fn test21() {
		// nums = [1,3], changeIndices = [1,1,1,2,1,1,1]
		let nums = vec![1,3];
		let change_indices = vec![1,1,1,2,1,1,1];
		let expected = 6;
		t1(nums, change_indices, expected);
	}

	#[test]
	fn test31() {
		// nums = [0,1], changeIndices = [2,2,2]
		let nums = vec![0, 1];
		let change_indices = vec![2, 2, 2];
		let expected = -1;
		t1(nums, change_indices, expected);
	}
}




