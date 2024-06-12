

#[cfg(test)]
mod tests {
	use crate::*;

	fn t(s: String, expected: String) {
		let result = Solution::reverse_vowels(s);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	#[test]
	fn test1() {
		let s = "hello".to_string();
		let expected = "holle".to_string();
		t(s, expected);
	}

	#[test]
	fn test2() {
		let s = "leetcode".to_string();
		let expected = "leotcede".to_string();
		t(s, expected);
	}
}