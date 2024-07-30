

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(arg: String, tr: String, expected: i32) {
		let result = Solution::find_permutation_difference(arg, tr);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t1(arg: String, tr: String, expected: i32) {
		let result = Solution1::find_permutation_difference(arg, tr);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	#[test]
	fn test1() {
		let s = "abc".to_string();
		let tr = "bca".to_string();
		let expected = 2;
		t(s, tr, expected);
	}

	#[test]
	fn test2() {
		let s = "abcde".to_string();
		let tr = "edbac".to_string();
		let expected = 12;
		t(s, tr, expected);
	}

	#[test]
	fn test11() {
		let s = "abc".to_string();
		let tr = "bca".to_string();
		let expected = 2;
		t1(s, tr, expected);
	}

	#[test]
	fn test21() {
		let s = "abcde".to_string();
		let tr = "edbac".to_string();
		let expected = 12;
		t1(s, tr, expected);
	}
}