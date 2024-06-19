

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(head: Option<Box<ListNode>>, expected: bool) {
		let result = Solution::is_palindrome(head);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let v = vec![1, 2, 2, 1];
		let head = helper(v);
		t(head, true);
	}

	#[test]
	fn test2() {
		let v = vec![1, 2];
		let head = helper(v);
		t(head, true);
	}

}