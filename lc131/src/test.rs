

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let s = "aab".to_string();
		let result = Solution::partition(s);
		println!("{:?}", result);
	}

	#[test]
	fn test2() {
		let s = "a".to_string();
		let result = Solution::partition(s);
		println!("{:?}", result);
	}

	#[test]
	fn test_is_palindrome() {
		println!("{:?}", Solution::is_palindrome("abdba".as_bytes()));
		println!("{:?}", Solution::is_palindrome("abba".as_bytes()));
		println!("{:?}", Solution::is_palindrome("aba".as_bytes()));
		println!("{:?}", Solution::is_palindrome("abc".as_bytes()));
		println!("{:?}", Solution::is_palindrome("a".as_bytes()));
		println!("{:?}", Solution::is_palindrome("".as_bytes()));
	}
}