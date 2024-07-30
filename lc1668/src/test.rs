

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(sequence: String, word: String, expected: i32) {
		let result = Solution::max_repeating(sequence, word);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t1(sequence: String, word: String, expected: i32) {
		let result = Solution1::max_repeating(sequence, word);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t2(sequence: String, word: String, expected: i32) {
		let result = Solution2::max_repeating(sequence, word);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let sequence = "ababc".to_string();
		let word = "ab".to_string();
		t(sequence, word, 2)
	}

	#[test]
	fn test2() {
		let sequence = "ababc".to_string();
		let word = "ba".to_string();
		t(sequence, word, 1)
	}

	#[test]
	fn test3() {
		let sequence = "ababc".to_string();
		let word = "ac".to_string();
		t(sequence, word, 0)
	}

	#[test]
	fn test4() {
		let sequence = "ababcabababa".to_string();
		let word = "ab".to_string();
		t(sequence, word, 3)
	}

	#[test]
	fn test5() {
		let sequence = "aaabaaaabaaabaaaabaaaabaaaabaaaaba".to_string();
		let word = "aaaba".to_string();
		t(sequence, word, 5)
	}

	#[test]
	fn test51() {
		let sequence = "aaabaaaabaaabaaaabaaaabaaaabaaaaba".to_string();
		let word = "aaaba".to_string();
		t1(sequence, word, 5)
	}

	#[test]
	fn test52() {
		let sequence = "aaabaaaabaaabaaaabaaaabaaaabaaaaba".to_string();
		let word = "aaaba".to_string();
		t2(sequence, word, 5)
	}

	#[test]
	fn test_strip_prefix() {
		let sq = "aaabaaaabaaabaaaabaaaabaaaabaaaaba".to_string();
		let word = "aaab".to_string();
		let apre = sq.strip_prefix(&word).unwrap();
		println!("{:?}", apre);

	}

	#[test]
	fn test_skip() {
		let sq = "aaabaaaabaaabaaaabaaaabaaaabaaaaba".to_string();
		let word = "aaaba".to_string();

		let s = sq.chars().skip(word.len() - 1)
			.map(|c| c.to_string())
			.collect::<String>();

		println!("{:?}", s);


	}

}