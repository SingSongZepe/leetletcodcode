

#[cfg(test)]
mod tests {
	use crate::*;

	fn t(amount: Vec<i32>, expected: i32) {
		let result = Solution::fill_cups(amount);
		println!("{}", result);
		assert_eq!(result, expected);
	}

	fn t1(amount: Vec<i32>, expected: i32) {
		let result = Solution1::fill_cups(amount);
		println!("{}", result);
		assert_eq!(result, expected);
	}


	#[test]
	fn test1() {
		let amount = vec![1, 4, 2];
		t(amount, 4);
	}

	#[test]
	fn test2() {
		let amount = vec![5, 4, 4];
		t(amount, 7);
	}

	#[test]
	fn test3() {
		let amount = vec![5, 0, 0];
		t(amount, 5);
	}

	#[test]
	fn test11() {
		let amount = vec![1, 4, 2];
		t1(amount, 4);
	}

	#[test]
	fn test21() {
		let amount = vec![5, 4, 4];
		t1(amount, 7);
	}

	#[test]
	fn test31() {
		let amount = vec![5, 0, 0];
		t1(amount, 5);
	}

	#[test]
	fn test41() {
		let amount = vec![0, 0, 0];
		t1(amount, 0);
	}
}