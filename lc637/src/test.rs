

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(root: Option<Rc<RefCell<TreeNode>>>, expected: Vec<f64>) {
		let result = Solution::average_of_levels(root);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let v = vec![3, 9, 20, -1, -1, 15, 7];
		let root = TreeNode::from_vec(&v);
		t(root, vec![3.0, 14.5, 11.0])
	}

	#[test]
	fn test_flatten() {
		let v = vec![vec![1], vec![2, 3], vec![4, 5, 6]];
		let result = v.iter().flatten().collect::<Vec<_>>();
		println!("{:?}", result);

		let opt = Some(Some(1));
		let result = opt.into_iter().flatten().collect::<Vec<_>>();
		println!("{:?}", result);
	}

}