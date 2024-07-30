

#[cfg(test)]
mod tests {
	use crate::*;

	fn t(s: String, expected: char) {
		let result = Solution::repeated_character(s);
		println!("{}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let s = "abccbaacz".to_string();
		t(s, 'c');
	}

	#[test]
	fn test2() {
		let s = "abcdd".to_string();
		t(s, 'd');
	}
}