

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(left: i32, right: i32, expected: i32) {
		let result = Solution::range_bitwise_and(left, right);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let left = 5;
		let right = 7;
		let expected = 4;
		t(left, right, expected);
	}

	#[test]
	fn test2() {
		let left = 0;
		let right = 0;
		let expected = 0;
		t(left, right, expected);
	}

	#[test]
	fn test3() {
		let left = 1;
		let right = 2147483647;
		let expected = 0;
		t(left, right, expected);
	}

}