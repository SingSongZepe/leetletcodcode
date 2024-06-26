

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(root: Node, expected: Node) {
		let result = Solution::convert_bst(root);
		println!("{:?}", result);
		// assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		// replace null with -1
		// root = [4,1,6,0,2,5,7,null,null,null,3,null,null,null,8]
		let v = vec![4,1,6,0,2,5,7,-1,-1,-1,3,-1,-1,-1,-1,8];
		let root = helper(v);
		t(root, None);
	}

	#[test]
	fn test2() {
		let v = vec![0, -1, 1];
		let root = helper(v);
		t(root, None);
	}

}