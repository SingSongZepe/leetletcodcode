

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(root: Option<Rc<RefCell<TreeNode>>>, expected: bool) {
		let result = Solution::is_symmetric(root);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let v = vec![1,2,2,3,4,4,3];
		let root = helper(v);
		t(root, true);
	}

	#[test]
	fn test2() {
		let v = vec![1,2,2,101,3,101,3];
		let root = helper(v);
		t(root, false);
	}

	#[test]
	fn test3() {
		let v = vec![1,2,2,3,101,-1,3];
		let root = helper(v);
		t(root, false);
	}
}