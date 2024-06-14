

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(arg: i32, expected: bool) {
		let result = Solution::can_win_nim(arg);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t1(arg: i32, expected: bool) {
		let result = Solution1::can_win_nim(arg);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	#[test]
	fn test1() {
		let n = 1;
		t(n, true);
	}

	#[test]
	fn test2() {
		let n = 4;
		t(n, false);
	}

	#[test]
	fn test3() {
		let n = 2;
		t(n, true);
	}

	#[test]
	fn test11() {
		let n = 1;
		t1(n, true);
	}

	#[test]
	fn test21() {
		let n = 4;
		t1(n, false);
	}

	#[test]
	fn test31() {
		let n = 2;
		t1(n, true);
	}
}