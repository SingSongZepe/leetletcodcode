

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let s = "12".to_string();
		let result = Solution::num_decodings(s);
	}

	#[test]
	fn test2() {
		let s = "226".to_string();
		let result = Solution::num_decodings(s);
	}

	#[test]
	fn test3() {
		let s = "06".to_string();
		let result = Solution::num_decodings(s);
	}
}