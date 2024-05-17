

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let root = helper(vec![1, 2, 3, 2, -1, 2, 4]);
		let target  = 2;
		let result = Solution::remove_leaf_nodes(root, target);
		println!("{}", if result.is_some() { result.unwrap().borrow().val.to_string() } else { "None".to_string() })
	}

	#[test]
	fn test2() {
		let root = helper(vec![1, 3, 3, 3, 2]);
		let target  = 3;
		let result = Solution::remove_leaf_nodes(root, target);
		println!("{}", if result.is_some() { result.unwrap().borrow().val.to_string() } else { "None".to_string() })
	}

	#[test]
	fn test3() {
		let root = helper(vec![1, 2, -1, 2 ,-1, 2]);
		let target  = 2;
		let result = Solution::remove_leaf_nodes(root, target);
		println!("{}", if result.is_some() { result.unwrap().borrow().val.to_string() } else { "None".to_string() })
	}

	#[test]
	fn test4() {
		let root = helper(vec![1, 1, 1]);
		let target  = 1;
		let result = Solution::remove_leaf_nodes(root, target);
		println!("{}", if result.is_some() { result.unwrap().borrow().val.to_string() } else { "None".to_string() })
	}

	// Solution 1
	#[test]
	fn test11() {
		let root = helper(vec![1, 2, 3, 2, -1, 2, 4]);
		let target  = 2;
		let result = Solution1::remove_leaf_nodes(root, target);
		println!("{}", if result.is_some() { result.unwrap().borrow().val.to_string() } else { "None".to_string() })
	}

	#[test]
	fn test21() {
		let root = helper(vec![1, 3, 3, 3, 2]);
		let target  = 3;
		let result = Solution1::remove_leaf_nodes(root, target);
		println!("{}", if result.is_some() { result.unwrap().borrow().val.to_string() } else { "None".to_string() })
	}

	#[test]
	fn test31() {
		let root = helper(vec![1, 2, -1, 2 ,-1, 2]);
		let target  = 2;
		let result = Solution1::remove_leaf_nodes(root, target);
		println!("{}", if result.is_some() { result.unwrap().borrow().val.to_string() } else { "None".to_string() })
	}

	#[test]
	fn test41() {
		let root = helper(vec![1, 1, 1]);
		let target  = 1;
		let result = Solution1::remove_leaf_nodes(root, target);
		println!("{}", if result.is_some() { result.unwrap().borrow().val.to_string() } else { "None".to_string() })
	}

	// Solution 2
	#[test]
	fn test12() {
		let root = helper(vec![1, 2, 3, 2, -1, 2, 4]);
		let target  = 2;
		let result = Solution2::remove_leaf_nodes(root, target);
		println!("{}", if result.is_some() { result.unwrap().borrow().val.to_string() } else { "None".to_string() })
	}

	#[test]
	fn test22() {
		let root = helper(vec![1, 3, 3, 3, 2]);
		let target  = 3;
		let result = Solution2::remove_leaf_nodes(root, target);
		println!("{}", if result.is_some() { result.unwrap().borrow().val.to_string() } else { "None".to_string() })
	}

	#[test]
	fn test32() {
		let root = helper(vec![1, 2, -1, 2 ,-1, 2]);
		let target  = 2;
		let result = Solution2::remove_leaf_nodes(root, target);
		println!("{}", if result.is_some() { result.unwrap().borrow().val.to_string() } else { "None".to_string() })
	}

	#[test]
	fn test42() {
		let root = helper(vec![1, 1, 1]);
		let target  = 1;
		let result = Solution2::remove_leaf_nodes(root, target);
		println!("{}", if result.is_some() { result.unwrap().borrow().val.to_string() } else { "None".to_string() })
	}

	// Solution 2
	#[test]
	fn test13() {
		let root = helper(vec![1, 2, 3, 2, -1, 2, 4]);
		let target  = 2;
		let result = Solution3::remove_leaf_nodes(root, target);
		println!("{}", if result.is_some() { result.unwrap().borrow().val.to_string() } else { "None".to_string() })
	}

	#[test]
	fn test23() {
		let root = helper(vec![1, 3, 3, 3, 2]);
		let target  = 3;
		let result = Solution3::remove_leaf_nodes(root, target);
		println!("{}", if result.is_some() { result.unwrap().borrow().val.to_string() } else { "None".to_string() })
	}

	#[test]
	fn test33() {
		let root = helper(vec![1, 2, -1, 2 ,-1, 2]);
		let target  = 2;
		let result = Solution3::remove_leaf_nodes(root, target);
		println!("{}", if result.is_some() { result.unwrap().borrow().val.to_string() } else { "None".to_string() })
	}

	#[test]
	fn test43() {
		let root = helper(vec![1, 1, 1]);
		let target  = 1;
		let result = Solution3::remove_leaf_nodes(root, target);
		println!("{}", if result.is_some() { result.unwrap().borrow().val.to_string() } else { "None".to_string() })
	}
}