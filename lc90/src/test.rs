

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let nums = vec![1, 2, 2];
		let result = Solution::subsets_with_dup(nums);
		println!("{:?}", result);
	}

	#[test]
	fn test2() {
		let nums = vec![0];
		let result = Solution::subsets_with_dup(nums);
		println!("{:?}", result);
	}

	#[test]
	fn test11() {
		let nums = vec![1, 2, 2];
		let result = Solution1::subsets_with_dup(nums);
		println!("{:?}", result);
	}

	#[test]
	fn test21() {
		let nums = vec![0];
		let result = Solution1::subsets_with_dup(nums);
		println!("{:?}", result);
	}

}