

#[cfg(test)]
mod tests {
	use crate::*;

	fn t(columnNumber: i32, expected: String) {
		let result = Solution::convert_to_title(columnNumber);
		assert_eq!(result, expected);
	}

	fn t1(columnNumber: i32, expected: String) {
		let result = Solution1::convert_to_title(columnNumber);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let columnNumber: i32 = 1;
		t(columnNumber, "A".to_string())
	}

	#[test]
	fn test2() {
		let columnNumber: i32 = 28;
		t(columnNumber, "AB".to_string())
	}

	#[test]
	fn test3() {
		let columnNumber: i32 = 701;
		t(columnNumber, "ZY".to_string())
	}

	#[test]
	fn test11() {
		let columnNumber: i32 = 1;
		t(columnNumber, "A".to_string())
	}

	#[test]
	fn test21() {
		let columnNumber: i32 = 28;
		t1(columnNumber, "AB".to_string())
	}

	#[test]
	fn test31() {
		let columnNumber: i32 = 701;
		t1(columnNumber, "ZY".to_string())
	}

}