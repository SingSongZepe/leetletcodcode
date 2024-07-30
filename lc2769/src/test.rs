

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(arg: i32, tr: i32, expected: i32) {
		let result = Solution::the_maximum_achievable_x(arg, tr);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let num = 4;
		let tr = 1;
		let expected = 6;
		t(num, tr, expected);
	}

	#[test]
	fn test2() {
		let num = 3;
		let tr = 2;
		let expected = 7;
		t(num, tr, expected);
	}

}