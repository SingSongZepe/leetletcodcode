

#[cfg(test)]
mod tests {
	use crate::*;

	fn t(s: String, target: String, expected: i32) {
		let result = Solution::rearrange_characters(s, target);
		println!("{}", result);
		assert_eq!(result, expected);
	}

	fn t1(s: String, target: String, expected: i32) {
		let result = Solution1::rearrange_characters(s, target);
		println!("{}", result);
		assert_eq!(result, expected);
	}

	#[test]
	fn test1() {
		let s = "ilovecodingonleetcode".to_string();
		let target = "code".to_string();
		let expected = 2;
		t(s, target, expected);
	}

	#[test]
	fn test2() {
		let s = "abcbc".to_string();
		let target = "abc".to_string();
		let expected = 1;
		t(s, target, expected);
	}

	#[test]
	fn test3() {
		let s = "aibc".to_string();
		let target = "abc".to_string();
		let expected = 1;
		t(s, target, expected);
	}

	#[test]
	fn test4() {
		let s = "abbaccaddaeea".to_string();
		let target = "aaaaa".to_string();
		let expected = 1;
		t(s, target, expected);
	}

	#[test]
	fn test5() {
		let s = "wenfxquitzryponxsikhciohyostvmkapkfpglzikitwiraqgchxnpryhwpuwpozacjhmwhjvslprqlnxrk".to_string();
		let target = "woijih".to_string();
		let expected = 2;
		t(s, target, expected);
	}

	#[test]
	fn test31() {
		let s = "aibc".to_string();
		let target = "abc".to_string();
		let expected = 1;
		t1(s, target, expected);
	}

	#[test]
	fn test41() {
		let s = "abbaccaddaeea".to_string();
		let target = "aaaaa".to_string();
		let expected = 1;
		t1(s, target, expected);
	}

	#[test]
	fn test51() {
		let s = "wenfxquitzryponxsikhciohyostvmkapkfpglzikitwiraqgchxnpryhwpuwpozacjhmwhjvslprqlnxrk".to_string();
		let target = "woijih".to_string();
		let expected = 2;
		t1(s, target, expected);
	}
}