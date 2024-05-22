

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let s = "aab".to_string();
		let result = Solution::min_cut(s);
		println!("{}", result);  // Output: 1
	}

	#[test]
	fn test2() {
		let s = "a".to_string();
		let result = Solution::min_cut(s);
		println!("{}", result); // Output: 0
	}

	#[test]
	fn test3() {
		let s = "ab".to_string();
		let result = Solution::min_cut(s);
		println!("{}", result); // Output: 1
	}

	#[test]
	fn test4() {
		let s = "ababababababababababababcbabababababababababababa".to_string();
		let result = Solution::min_cut(s);
		println!("{}", result); // Output: 1
	}

	// solution 1
	#[test]
	fn test11() {
		let s = "aab".to_string();
		let result = Solution1::min_cut(s);
		println!("{}", result);  // Output: 1
	}

	#[test]
	fn test21() {
		let s = "a".to_string();
		let result = Solution1::min_cut(s);
		println!("{}", result); // Output: 0
	}

	#[test]
	fn test31() {
		let s = "ab".to_string();
		let result = Solution1::min_cut(s);
		println!("{}", result); // Output: 1
	}

	#[test]
	fn test41() {
		let s = "ababababababababababababcbabababababababababababa".to_string();
		let result = Solution1::min_cut(s);
		println!("{}", result); // Output: 1
	}

	// solution 1
	#[test]
	fn test12() {
		let s = "aab".to_string();
		let result = Solution2::min_cut(s);
		println!("{}", result);  // Output: 1
	}

	#[test]
	fn test22() {
		let s = "a".to_string();
		let result = Solution2::min_cut(s);
		println!("{}", result); // Output: 0
	}

	#[test]
	fn test32() {
		let s = "ab".to_string();
		let result = Solution2::min_cut(s);
		println!("{}", result); // Output: 1
	}

	#[test]
	fn test42() {
		let s = "ababababababababababababcbabababababababababababa".to_string();
		let result = Solution2::min_cut(s);
		println!("{}", result); // Output: 1
	}
}