

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let n = 2;
		let result = Solution::gray_code(n);
		println!("{:?}", result);
	}

	#[test]
	fn test2() {
		let n = 1;
		let result = Solution::gray_code(n);
		println!("{:?}", result);
	}

	#[test]
	fn test3() {
		let n = 3;
		let result = Solution::gray_code(n);
		println!("{:?}", result);
	}

	#[test]
	fn test4() {
		let n = 4;
		let result = Solution::gray_code(n);
		println!("{:?}", result);
	}

	#[test]
	fn test11() {
		let n = 2;
		let result = Solution1::gray_code(n);
		println!("{:?}", result);
	}

	#[test]
	fn test21() {
		let n = 1;
		let result = Solution1::gray_code(n);
		println!("{:?}", result);
	}

	#[test]
	fn test31() {
		let n = 3;
		let result = Solution1::gray_code(n);
		println!("{:?}", result);
	}

	#[test]
	fn test41() {
		let n = 4;
		let result = Solution1::gray_code(n);
		println!("{:?}", result);
	}
}