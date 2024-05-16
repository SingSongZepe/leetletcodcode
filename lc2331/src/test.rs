

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let v = vec![2,1,3,-1,-1,0,1];
		let root = helper(v);
		let result = Solution::evaluate_tree(root);
		println!("{}", result);
	}

	#[test]
	fn test2() {
		let v = vec![0];
		let root = helper(v);
		let result = Solution::evaluate_tree(root);
		println!("{}", result);
	}

	// solution 1
	#[test]
	fn test11() {
		let v = vec![2,1,3,-1,-1,0,1];
		let root = helper(v);
		let result = Solution1::evaluate_tree(root);
		println!("{}", result);
	}

	#[test]
	fn test21() {
		let v = vec![0];
		let root = helper(v);
		let result = Solution1::evaluate_tree(root);
		println!("{}", result);
	}
}