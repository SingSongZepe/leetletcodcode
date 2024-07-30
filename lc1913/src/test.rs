

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let nums = vec![5, 6, 2, 7, 4];
		let result = Solution::max_product_difference(nums);
		println!("{}", result);
	}

	#[test]
	fn test2() {
		let nums = vec![4, 2, 5, 9, 7, 4, 8];
		let result = Solution::max_product_difference(nums);
		println!("{}", result);
	}

	//
	#[test]
	fn test11() {
		let nums = vec![5, 6, 2, 7, 4];
		let result = Solution1::max_product_difference(nums);
		println!("{}", result);
	}

	#[test]
	fn test21() {
		let nums = vec![4, 2, 5, 9, 7, 4, 8];
		let result = Solution1::max_product_difference(nums);
		println!("{}", result);
	}

	// Solution 2
	#[test]
	fn test12() {
		let nums = vec![5, 6, 2, 7, 4];
		let result = Solution2::max_product_difference(nums);
		println!("{}", result);
	}

	#[test]
	fn test22() {
		let nums = vec![4, 2, 5, 9, 7, 4, 8];
		let result = Solution2::max_product_difference(nums);
		println!("{}", result);
	}
}