

#[cfg(test)]
mod tests {
	use crate::*;

	fn t(n: i32, expected: bool) {
		let result = Solution::is_happy(n);
		println!("{}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let n = 19;
		t(n, true);
	}

	#[test]
	fn test2() {
		let n = 2;
		t(n, false);
	}
}