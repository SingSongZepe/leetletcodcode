

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(pattern: String, s: String, expected: bool) {
		let result = Solution::word_pattern(pattern, s);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let pattern = "abba".to_string();
		let s = "dog cat cat dog".to_string();
		let expected = true;
		t(pattern, s, expected);
	}

	#[test]
	fn test2() {
		let pattern = "abba".to_string();
		let s = "dog cat cat fish".to_string();
		let expected = false;
		t(pattern, s, expected);
	}

	#[test]
	fn test3() {
		let pattern = "aaaa".to_string();
		let s = "dog cat cat dog".to_string();
		let expected = false;
		t(pattern, s, expected);
	}

	#[test]
	fn test4() {
		let pattern = "abba".to_string();
		let s = "dog dog dog dog".to_string();
		let expected = false;
		t(pattern, s, expected);
	}

	#[test]
	fn test5() {
		let pattern = "aba".to_string();
		let s = "cat dog dog".to_string();
		let expected = false;
		t(pattern, s, expected);
	}

	#[test]
	fn test6() {
		let pattern = "aaa".to_string();
		let s = "aa aa aa aa".to_string();
		let expected = false;
		t(pattern, s, expected);
	}

	#[test]
	fn test_bitree() {
		let mut bimap = BiMap::new();

		bimap.insert("key1", "value1");
		bimap.insert("key2", "value2");

		println!("{:?}", bimap.get_value(&"key1")); // Some("value1")
		println!("{:?}", bimap.get_key(&"value2")); // Some("key2")

	}

}