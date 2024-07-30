

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(paragraph: String, banned: Vec<String>, expected: String) {
		let result = Solution::most_common_word(paragraph, banned);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let paragraph = "Bob hit a ball, the hit BALL flew far after it was hit.".to_string();
		let banned = vec!["hit".to_string()];
		let expected = "ball".to_string();
		t(paragraph, banned, expected);
	}

	#[test]
	fn test2() {
		let paragraph = "a.".to_string();
		let banned = vec![];
		let expected = "a".to_string();
		t(paragraph, banned, expected);
	}

	#[test]
	fn test3() {
		let paragraph = "Bob".to_string();
		let banned = vec![];
		let expected = "Bob".to_string();
		t(paragraph, banned, expected);
	}

}