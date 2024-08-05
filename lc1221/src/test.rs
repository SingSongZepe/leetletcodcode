

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(arg: ___, expected: ___) {
		let result = Solution::balanced_string_split(arg);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	fn t1(arg: ___, expected: ___) {
		let result = Solution1::balanced_string_spl(arg);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	#[test]
	fn test1() {
		let s = "RLRRLLRLRL".to_string();
		t(s, 4)
	}

	#[test]
	fn test2() {
		let s = "RLRRRLLRLL".to_string();
		t(s, 2)
	}

	#[test]
	fn test3() {
		let s = "LLLLRRRR".to_string();
		t(s, 1)
	}	

}