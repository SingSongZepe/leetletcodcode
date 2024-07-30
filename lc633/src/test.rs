

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(arg: i32, expected: bool) {
		let result = Solution::judge_square_sum(arg);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t1(arg: i32, expected: bool) {
		let result = Solution1::judge_square_sum(arg);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t2(arg: i32, expected: bool) {
		let result = Solution2::judge_square_sum(arg);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let c = 5;
		t(c, true);
	}

	#[test]
	fn test2() {
		let c = 3;
		t(c, false);
	}

	#[test]
	fn test3() {
		let c = 8;
		t(c, true);
	}

	#[test]
	fn test11() {
		let c = 5;
		t1(c, true);
	}

	#[test]
	fn test21() {
		let c = 3;
		t1(c, false);
	}

	#[test]
	fn test31() {
		let c = 8;
		t1(c, true);
	}


	#[test]
	fn test12() {
		let c = 5;
		t2(c, true);
	}

	#[test]
	fn test22() {
		let c = 3;
		t2(c, false);
	}

	#[test]
	fn test32() {
		let c = 8;
		t2(c, true);
	}
}