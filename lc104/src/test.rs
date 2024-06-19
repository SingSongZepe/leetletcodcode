

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(root: Option<Rc<RefCell<TreeNode>>>, expected: i32) {
		let result = Solution::max_depth(root);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t1(root: Option<Rc<RefCell<TreeNode>>>, expected: i32) {
		let result = Solution1::max_depth(root);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	#[test]
	fn test1() {
		let v = vec![3,9,20,101,101,15,7];
		let root = helper(v);
		t(root, 3);
	}

	#[test]
	fn test2() {
		let v = vec![1,101,2];
		let root = helper(v);
		t(root, 2);
	}

	#[test]
	fn test11() {
		let v = vec![3,9,20,101,101,15,7];
		let root = helper(v);
		t1(root, 3);
	}

	#[test]
	fn test21() {
		let v = vec![1,101,2];
		let root = helper(v);
		t1(root, 2);
	}

}