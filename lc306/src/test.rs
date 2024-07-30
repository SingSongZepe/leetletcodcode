

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(arg: String, expected: bool) {
		let result = Solution::is_additive_number(arg);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let arg = "112358".to_string();
		let expected = true;
		t(arg, expected);
	}

	#[test]
	fn test2() {
		let arg = "199100199".to_string();
		let expected = true;
		t(arg, expected);
	}

	#[test]
	fn test3() {
		let arg = "123".to_string();
		let expected = true;
		t(arg, expected);
	}

	#[test]
	fn test4() {
		let arg = "1023".to_string();
		let expected = false;
		t(arg, expected);
	}

	#[test]
	fn test5() {
		let arg = "1999999999910000000000".to_string();
		let expected = true;
		t(arg, expected);
	}

}