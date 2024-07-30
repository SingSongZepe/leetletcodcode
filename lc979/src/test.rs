

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let v = vec![3, 0, 0];
		let root = helper(v);
		let result = Solution::distribute_coins(root);
		println!("{}", result);
	}

	#[test]
	fn test2() {
		let v = vec![0, 3, 0];
		let root = helper(v);
		let result = Solution::distribute_coins(root);
		println!("{}", result);
	}

	#[test]
	fn test3() {
		let v = vec![3, 4, 0, 0, 0, 0, 0];
		let root = helper(v);
		let result = Solution::distribute_coins(root);
		println!("{}", result);
	}

	// solution 1
	#[test]
	fn test11() {
		let v = vec![3, 0, 0];
		let root = helper(v);
		let result = Solution1::distribute_coins(root);
		println!("{}", result);
	}

	#[test]
	fn test21() {
		let v = vec![0, 3, 0];
		let root = helper(v);
		let result = Solution1::distribute_coins(root);
		println!("{}", result);
	}

	#[test]
	fn test31() {
		let v = vec![3, 4, 0, 0, 0, 0, 0];
		let root = helper(v);
		let result = Solution1::distribute_coins(root);
		println!("{}", result);
	}
}