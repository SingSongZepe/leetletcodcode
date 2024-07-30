

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(n: i32, expected: i32) {
		let result = Solution::total_money(n);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let n = 4;
		let expected = 10;
		t(n, expected);
	}

	#[test]
	fn test2() {
		let n = 10;
		let expected = 37;
		t(n, expected);
	}

	#[test]
	fn test3() {
		let n = 20;
		let expected = 96;
		t(n, expected);
	}
}