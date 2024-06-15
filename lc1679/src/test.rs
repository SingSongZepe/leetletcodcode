

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(arg: Vec<i32>, k: i32, expected: i32) {
		let result = Solution::max_operations(arg, k);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t1(arg: Vec<i32>, k: i32, expected: i32) {
		let result = Solution1::max_operations(arg, k);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let nums = vec![1,2,3,4];
		let k = 5;
		let expected = 2;
		t(nums, k, expected);
	}

	#[test]
	fn test2() {
		let nums = vec![3,1,3,4,3];
		let k = 6;
		let expected = 1;
		t(nums, k, expected);
	}

	#[test]
	fn test11() {
		let nums = vec![1,2,3,4];
		let k = 5;
		let expected = 2;
		t1(nums, k, expected);
	}

	#[test]
	fn test21() {
		let nums = vec![3,1,3,4,3];
		let k = 6;
		let expected = 1;
		t1(nums, k, expected);
	}

}