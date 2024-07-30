

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let s = "0100".to_string();
		let result = Solution::min_operations(s);
		println!("{}", result);
	}

	#[test]
	fn test2() {
		let s = "10".to_string();
		let result = Solution::min_operations(s);
		println!("{}", result);
	}

	#[test]
	fn test3() {
		let s = "1111".to_string();
		let result = Solution::min_operations(s);
		println!("{}", result);
	}

}