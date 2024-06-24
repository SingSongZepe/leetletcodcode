

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(nums: Vec<i32>, k: i32, expected: i32) {
		let result = Solution::min_k_bit_flips(nums, k);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let nums = vec![0, 1, 0];
		let k = 1;
		t(nums, k, 2)
	}

	#[test]
	fn test2() {
		let nums = vec![1, 1, 0];
		let k = 2;
		t(nums, k, 1)
	}

	#[test]
	fn test3() {
		let nums = vec![0, 0, 0, 1, 0, 1, 1, 0];
		let k = 3;
		t(nums, k, 3)
	}
}