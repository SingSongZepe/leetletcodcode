

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(arg: String, str2: String, expected: String) {
		let result = Solution::gcd_of_strings(arg, str2);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let str1 = "ABCABC".to_string();
		let str2 = "ABC".to_string();
		let expected = "ABC".to_string();
		t(str1, str2, expected);
	}

	#[test]
	fn test2() {
		let str1 = "ABABAB".to_string();
		let str2 = "ABAB".to_string();
		let expected = "AB".to_string();
		t(str1, str2, expected);
	}

	#[test]
	fn test3() {
		let str1 = "LEET".to_string();
		let str2 = "CODE".to_string();
		let expected = "".to_string();
		t(str1, str2, expected);
	}

}