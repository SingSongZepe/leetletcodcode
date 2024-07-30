

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let nums = vec![1, 3];
		let result = Solution::subset_xor_sum(nums);
		println!("{}", result);
	}

	#[test]
	fn test2() {
		let nums = vec![5, 1, 6];
		let result = Solution::subset_xor_sum(nums);
		println!("{}", result);
	}

	#[test]
	fn test3() {
		let nums = vec![3, 4, 5, 6, 7, 8];
		let result = Solution::subset_xor_sum(nums);
		println!("{}", result);
	}

	// Solution 1
	#[test]
	fn test11() {
		let nums = vec![1, 3];
		let result = Solution1::subset_xor_sum(nums);
		println!("{}", result);
	}

	#[test]
	fn test21() {
		let nums = vec![5, 1, 6];
		let result = Solution1::subset_xor_sum(nums);
		println!("{}", result);
	}

	#[test]
	fn test31() {
		let nums = vec![3, 4, 5, 6, 7, 8];
		let result = Solution1::subset_xor_sum(nums);
		println!("{}", result);
	}
}