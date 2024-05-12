

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let s = "12".to_string();
		let result = Solution::num_decodings(s);
		println!("{}", result);
	}

	#[test]
	fn test2() {
		let s = "226".to_string();
		let result = Solution::num_decodings(s);
		println!("{}", result);
	}

	#[test]
	fn test3() {
		let s = "06".to_string();
		let result = Solution::num_decodings(s);
		println!("{}", result);
	}

	#[test]
	fn test11() {
		let s = "12".to_string();
		let result = Solution1::num_decodings(s);
		println!("{}", result);
	}

	#[test]
	fn test21() {
		let s = "226".to_string();
		let result = Solution1::num_decodings(s);
		println!("{}", result);
	}

	#[test]
	fn test31() {
		let s = "06".to_string();
		let result = Solution1::num_decodings(s);
		println!("{}", result);
	}
}