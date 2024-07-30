

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let head = helper(vec![1, 4, 3, 2, 5, 2]);
		let x = 3;
		let result = Solution::partition(head, x);
		print_list(&result);
	}

	#[test]
	fn test2() {
		let head = helper(vec![2, 1]);
		let x = 2;
		let result = Solution::partition(head, x);
		print_list(&result);
	}

	#[test]
	fn test3() {
		let head = helper(vec![]);
		let x = 0;
		let result = Solution::partition(head, x);
		print_list(&result);
	}

	// Solution 1
	#[test]
	fn test11() {
		let head = helper(vec![1, 4, 3, 2, 5, 2]);
		let x = 3;
		let result = Solution1::partition(head, x);
		print_list(&result);
	}

	#[test]
	fn test21() {
		let head = helper(vec![2, 1]);
		let x = 2;
		let result = Solution1::partition(head, x);
		print_list(&result);
	}

	#[test]
	fn test31() {
		let head = helper(vec![]);
		let x = 0;
		let result = Solution1::partition(head, x);
		print_list(&result);
	}

	// Solution 2
	#[test]
	fn test12() {
		let head = helper(vec![1, 4, 3, 2, 5, 2]);
		let x = 3;
		let result = Solution2::partition(head, x);
		print_list(&result);
	}

	#[test]
	fn test22() {
		let head = helper(vec![2, 1]);
		let x = 2;
		let result = Solution2::partition(head, x);
		print_list(&result);
	}
	#[test]
	fn test32() {
		let head = helper(vec![]);
		let x = 0;
		let result = Solution2::partition(head, x);
		print_list(&result);
	}
}