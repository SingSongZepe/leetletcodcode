

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(pushed: Vec<i32>, popped: Vec<i32>, expected: bool) {
		let result = Solution::validate_stack_sequences(pushed, popped);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t1(pushed: Vec<i32>, popped: Vec<i32>, expected: bool) {
		let result = Solution1::validate_stack_sequences(pushed, popped);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	#[test]
	fn test1() {
		let pushed = vec![1, 2, 3, 4, 5];
		let popped = vec![4, 5, 3, 2, 1];
		let expected = true;
		t(pushed, popped, expected);
	}

	#[test]
	fn test2() {
		let pushed = vec![1, 2, 3, 4, 5];
		let popped = vec![4, 3, 5, 1, 2];
		let expected = false;
		t(pushed, popped, expected);
	}

	#[test]
	fn test3() {
		let pushed = vec![4,0,1,2,3];
		let popped = vec![4,2,3,0,1];
		let expected = false;
		t(pushed, popped, expected);
	}

	#[test]
	fn test11() {
		let pushed = vec![1, 2, 3, 4, 5];
		let popped = vec![4, 5, 3, 2, 1];
		let expected = true;
		t1(pushed, popped, expected);
	}

	#[test]
	fn test21() {
		let pushed = vec![1, 2, 3, 4, 5];
		let popped = vec![4, 3, 5, 1, 2];
		let expected = false;
		t1(pushed, popped, expected);
	}

	#[test]
	fn test31() {
		let pushed = vec![4,0,1,2,3];
		let popped = vec![4,2,3,0,1];
		let expected = false;
		t1(pushed, popped, expected);
	}
}