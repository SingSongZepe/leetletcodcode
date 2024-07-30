

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let mut root = helper(vec![1, 3, -1, -1, 2]);
		let collected = collect_tree(&root);
		println!("{:?}", collected);
		Solution::recover_tree(&mut root);
		let collected = collect_tree(&root);
		println!("{:?}", collected);
	}

	// Solution 1
	#[test]
	fn test11() {
		let mut root = helper(vec![1, 3, -1, -1, 2]);
		let collected = collect_tree(&root);
		println!("{:?}", collected);
		Solution1::recover_tree(&mut root);
		let collected = collect_tree(&root);
		println!("{:?}", collected);
	}


	#[test]
	fn test_collect_tree() {
		let mut root = helper(vec![3, 1, -1, -1, 2]);
		let collected = collect_tree(&root);
		println!("{:?}", collected);
	}


}