

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(n: i32, expected: i32) {
		let solution = Solution::new();
		let result = solution.first_bad_version(n);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let n = 5;
		let expected = 4;
		t(n, expected);
	}

	#[test]
	fn test2() {
		let n = 1;
		let expected = 1;
		t(n, expected);
	}



	#[test]
	fn test3() {
		let n = 2126753390;
		let expected = 1702766719;
		t(n, expected);
	}


}