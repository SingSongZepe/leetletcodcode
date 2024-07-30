

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let s = "011101".to_string();
		let result = Solution::max_score(s);
		println!("{:?}", result);
	}

	#[test]
	fn test2() {
		let s = "00111".to_string();
		let result = Solution::max_score(s);
		println!("{:?}", result);
	}

	#[test]
	fn test3() {
		let s = "1111".to_string();
		let result = Solution::max_score(s);
		println!("{:?}", result);
	}

	#[test]
	fn test4() {
		let s = "00".to_string();
		let result = Solution::max_score(s);
		println!("{:?}", result);
	}

	#[test]
	fn test5() {
		let s = "01001".to_string();
		let result = Solution::max_score(s);
		println!("{:?}", result);
	}

	// solution 1
	#[test]
	fn test11() {
		let s = "011101".to_string();
		let result = Solution1::max_score(s);
		println!("{:?}", result);
	}

	#[test]
	fn test21() {
		let s = "00111".to_string();
		let result = Solution1::max_score(s);
		println!("{:?}", result);
	}

	#[test]
	fn test31() {
		let s = "1111".to_string();
		let result = Solution1::max_score(s);
		println!("{:?}", result);
	}

	#[test]
	fn test41() {
		let s = "00".to_string();
		let result = Solution1::max_score(s);
		println!("{:?}", result);
	}

	#[test]
	fn test51() {
		let s = "01001".to_string();
		let result = Solution1::max_score(s);
		println!("{:?}", result);
	}
}