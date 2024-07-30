

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(arg: String, tr: String, expected: char) {
		let result = Solution::find_the_difference(arg, tr);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let s = "abcd".to_string();
		let tr = "abcde".to_string();
		let expected = 'e';
		t(s, tr, expected);
	}

	#[test]
	fn test2() {
		let s = "".to_string();
		let tr = "y".to_string();
		let expected = 'y';
		t(s, tr, expected);
	}

}