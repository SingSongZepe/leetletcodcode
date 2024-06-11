

#[cfg(test)]
mod tests {
	use crate::*;

	fn t(n: i32, expected: i32) {
		let result = Solution::hamming_weight(n);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let n = 11;
		t(n, 3)
	}

	#[test]
	fn test2() {
		let n = 128;
		t(n, 1)
	}

	#[test]
	fn test3() {
		let n = 2147483645;
		t(n, 30)
	}

}