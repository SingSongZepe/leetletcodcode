

#[cfg(test)]
mod tests {
	use crate::*;

	fn t(nums: Vec<i32>, k: i32, expected: i32) {
		let result = Solution::sum_indices_with_k_set_bits(nums, k);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t1(nums: Vec<i32>, k: i32, expected: i32) {
		let result = Solution1::sum_indices_with_k_set_bits(nums, k);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let nums = vec![5, 10, 1, 5, 2];
		let k = 1;
		t(nums, k, 13);
	}

	#[test]
	fn test2() {
		let nums = vec![4, 3, 2, 1];
		let k = 2;
		t(nums, k, 1);
	}

	#[test]
	fn test11() {
		let nums = vec![5, 10, 1, 5, 2];
		let k = 1;
		t1(nums, k, 13);
	}

	#[test]
	fn test21() {
		let nums = vec![4, 3, 2, 1];
		let k = 2;
		t1(nums, k, 1);
	}
}