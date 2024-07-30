

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let s = "anagram".to_string();
		let t = "nagaram".to_string();
		let result = Solution::is_anagram(s, t);
		println!("{}", result);
	}

	#[test]
	fn test2() {
		let s = "rat".to_string();
		let t = "car".to_string();
		let result = Solution::is_anagram(s, t);
		println!("{}", result);
	}

	#[test]
	fn test3() {
		let s = "ra".to_string();
		let t = "car".to_string();
		let result = Solution::is_anagram(s, t);
		println!("{}", result);
	}

	#[test]
	fn test4() {
		let s = "a".to_string();
		let t = "ab".to_string();
		let result = Solution::is_anagram(s, t);
		println!("{}", result);
	}

	// solution 1
	#[test]
	fn test11() {
		let s = "anagram".to_string();
		let t = "nagaram".to_string();
		let result = Solution1::is_anagram(s, t);
		println!("{}", result);
	}

	#[test]
	fn test21() {
		let s = "rat".to_string();
		let t = "car".to_string();
		let result = Solution1::is_anagram(s, t);
		println!("{}", result);
	}

	#[test]
	fn test31() {
		let s = "ra".to_string();
		let t = "car".to_string();
		let result = Solution1::is_anagram(s, t);
		println!("{}", result);
	}

	#[test]
	fn test41() {
		let s = "a".to_string();
		let t = "ab".to_string();
		let result = Solution1::is_anagram(s, t);
		println!("{}", result);
	}
}