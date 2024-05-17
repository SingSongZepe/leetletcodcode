

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let root = helper(make_vec_opt(vec![1, 7, 0, 7, -8, i32::MIN, i32::MIN]));
		let result = Solution::max_level_sum(root);
		println!("{}", result);
	}

	#[test]
	fn test2() {
		let root = helper(make_vec_opt(vec![989, i32::MIN, 10250, i32::MIN, i32::MIN, 98693, -89388, i32::MIN, i32::MIN, i32::MIN, i32::MIN, i32::MIN, i32::MIN, i32::MIN, -32768]));
		let result = Solution::max_level_sum(root);
		println!("{}", result);
	}

	#[test]
	fn test3() {
		let root = helper(make_vec_opt(vec![989, i32::MIN, 10250, i32::MIN, i32::MIN, 98693, -89388, i32::MIN, i32::MIN, i32::MIN, i32::MIN, i32::MIN, i32::MIN, i32::MIN, 22500]));
		let result = Solution::max_level_sum(root);
		println!("{}", result);
	}


	// solution 1 find the layer
	#[test]
	fn test11() {
		let root = helper(make_vec_opt(vec![1, 7, 0, 7, -8, i32::MIN, i32::MIN]));
		let result = Solution1::max_level_sum(root);
		println!("{}", result);
	}

	#[test]
	fn test21() {
		let root = helper(make_vec_opt(vec![989, i32::MIN, 10250, i32::MIN, i32::MIN, 98693, -89388, i32::MIN, i32::MIN, i32::MIN, i32::MIN, i32::MIN, i32::MIN, i32::MIN, -32768]));
		let result = Solution1::max_level_sum(root);
		println!("{}", result);
	}

	#[test]
	fn test31() {
		let root = helper(make_vec_opt(vec![989, i32::MIN, 10250, i32::MIN, i32::MIN, 98693, -89388, i32::MIN, i32::MIN, i32::MIN, i32::MIN, i32::MIN, i32::MIN, i32::MIN, 22500]));
		let result = Solution1::max_level_sum(root);
		println!("{}", result);
	}

	// solution 2
	#[test]
	fn test12() {
		let root = helper(make_vec_opt(vec![1, 7, 0, 7, -8, i32::MIN, i32::MIN]));
		let result = Solution2::max_level_sum(root);
		println!("{}", result);
	}

	#[test]
	fn test22() {
		let root = helper(make_vec_opt(vec![989, i32::MIN, 10250, i32::MIN, i32::MIN, 98693, -89388, i32::MIN, i32::MIN, i32::MIN, i32::MIN, i32::MIN, i32::MIN, i32::MIN, -32768]));
		let result = Solution2::max_level_sum(root);
		println!("{}", result);
	}

	#[test]
	fn test32() {
		let root = helper(make_vec_opt(vec![989, i32::MIN, 10250, i32::MIN, i32::MIN, 98693, -89388, i32::MIN, i32::MIN, i32::MIN, i32::MIN, i32::MIN, i32::MIN, i32::MIN, 22500]));
		let result = Solution2::max_level_sum(root);
		println!("{}", result);
	}
}