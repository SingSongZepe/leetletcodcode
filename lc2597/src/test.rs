

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let nums = vec![2, 4, 6];
		let k = 2;
		let result = Solution::beautiful_subsets(nums, k);
		println!("{}", result); // expected 4
	}

	#[test]
	fn test2() {
		let nums = vec![1];
		let k = 2;
		let result = Solution::beautiful_subsets(nums, k);
		println!("{}", result);
	}

	#[test]
	fn test3() {
		let nums = vec![10,4,5,7,2,1];
		let k = 3;
		let result = Solution::beautiful_subsets(nums, k);
		println!("{}", result);
	}

	#[test]
	fn test4() {
		let nums = vec![2, 2, 2];
		let k = 0;
		let result = Solution::beautiful_subsets(nums, k);
		println!("{}", result);
	}

	#[test]
	fn test5() {
		let nums = vec![1000,1000,1000,1000,1000,1000,1000,1000,1000,1000,1000,1000,1000,1000,1000,1000,1000,1000,1000,1000];
		let k = 1;
		let result = Solution::beautiful_subsets(nums, k);
		println!("{}", result);
	}

	#[test]
	fn test_why_we_need_to_sort() {
		let nums = vec![18, 12, 10, 5, 6, 2, 18, 3];
		let k = 8;
		let result = Solution::beautiful_subsets_unsorted(nums, k);
		println!("{}", result);
	}
}