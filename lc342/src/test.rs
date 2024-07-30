

#[cfg(test)]
mod tests {
	use crate::*;

	fn t(n: i32, expected: bool) {
		let result = Solution::is_power_of_four(n);
		println!("{}", result);
		assert_eq!(result, expected);
	}

	fn t1(n: i32, expected: bool) {
		let result = Solution1::is_power_of_four(n);
		println!("{}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let n = 16;
		let expected = true;
		t(n, expected);
	}

	#[test]
	fn test2() {
		let n = 5;
		let expected = false;
		t(n, expected);
	}

	#[test]
	fn test3() {
		let n = 1;
		let expected = true;
		t(n, expected);
	}

	#[test]
	fn test11() {
		let n = 16;
		let expected = true;
		t1(n, expected);
	}

	#[test]
	fn test21() {
		let n = 5;
		let expected = false;
		t1(n, expected);
	}

	#[test]
	fn test31() {
		let n = 1;
		let expected = true;
		t1(n, expected);
	}
}