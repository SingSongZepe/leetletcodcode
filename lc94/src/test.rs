

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let v = vec![1, -1, 2, -1, -1, 3];
		let root = helper(v);
		let result = Solution::inorder_traversal(root);
		println!("{:?}", result);
	}

	#[test]
	fn test2() {
		let v = vec![];
		let root = helper(v);
		let result = Solution::inorder_traversal(root);
		println!("{:?}", result);
	}

	#[test]
	fn test3() {
		let v = vec![1];
		let root = helper(v);
		let result = Solution::inorder_traversal(root);
		println!("{:?}", result);
	}

	#[test]
	fn test11() {
		let v = vec![1, -1, 2, -1, -1, 3];
		let root = helper(v);
		let result = Solution1::inorder_traversal(root);
		println!("{:?}", result);
	}

	#[test]
	fn test21() {
		let v = vec![];
		let root = helper(v);
		let result = Solution1::inorder_traversal(root);
		println!("{:?}", result);
	}

	#[test]
	fn test31() {
		let v = vec![1];
		let root = helper(v);
		let result = Solution1::inorder_traversal(root);
		println!("{:?}", result);
	}
}