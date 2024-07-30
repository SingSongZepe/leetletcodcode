

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let nums = vec![1, 2, 1];
		let k = 3;
		let edges = vec![vec![0, 1], vec![0, 2]];
		let result = Solution::maximum_value_sum(nums, k, edges);
		println!("{}", result);
	}

	#[test]
	fn test2() {
		let nums = vec![2, 3];
		let k = 7;
		let edges = vec![vec![0, 1]];
		let result = Solution::maximum_value_sum(nums, k, edges);
		println!("{}", result);
	}

	#[test]
	fn test3() {
		let nums = vec![7, 7, 7, 7, 7, 7];
		let k = 3;
		let edges = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5]];
		let result = Solution::maximum_value_sum(nums, k, edges);
		println!("{}", result);
	}
}