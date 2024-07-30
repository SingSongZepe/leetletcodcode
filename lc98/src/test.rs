

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let v = vec![2, 1, 3];
		let tree = helper(v);
		let result = Solution::is_valid_bst(tree);
		println!("{}", result);
	}

	#[test]
	fn test2() {
		let v = vec![5, 1, 4, -1, -1 ,3, 6];
		let tree = helper(v);
		let result = Solution::is_valid_bst(tree);
		println!("{}", result);
	}
}