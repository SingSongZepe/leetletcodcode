

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(arg: Vec<i32>, k: i32, expected: i32) {
		let result = Solution::min_operations(arg, k);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t1(arg: Vec<i32>, k: i32, expected: i32) {
		let result = Solution1::min_operations(arg, k);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let nums = vec![3, 1, 5, 4, 2];
		let k = 2;
		t(nums, k, 4);
	}

	#[test]
	fn test2() {
		let nums = vec![3,1,5,4,2];
		let k = 5;
		t(nums, k, 5);
	}

	#[test]
	fn test3() {
		let nums = vec![3,2,5,4,1];
		let k = 3;
		t(nums, k, 5);
	}


	#[test]
	fn test11() {
		let nums = vec![3, 1, 5, 4, 2];
		let k = 2;
		t1(nums, k, 4);
	}

	#[test]
	fn test21() {
		let nums = vec![3,1,5,4,2];
		let k = 5;
		t1(nums, k, 5);
	}

	#[test]
	fn test31() {
		let nums = vec![3,2,5,3,1];
		let k = 3;
		t1(nums, k, 4);
	}
}