

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let p = vec![1, 2, 3];
		let q = vec![1, 2, 3];
		let rootp = helper(p);
		let rootq = helper(q);
		let result = Solution::is_same_tree(rootp, rootq);
		println!("{:?}", result);
	}

	#[test]
	fn test2() {
		let p = vec![1, 2];
		let q = vec![1, -1, 2];
		let rootp = helper(p);
		let rootq = helper(q);
		let result = Solution::is_same_tree(rootp, rootq);
		println!("{:?}", result);
	}

	#[test]
	fn test3() {
		let p = vec![1, 2, 1];
		let q = vec![1, 1, 2];
		let rootp = helper(p);
		let rootq = helper(q);
		let result = Solution::is_same_tree(rootp, rootq);
		println!("{:?}", result);
	}

	// solution 1
	#[test]
	fn test11() {
		let p = vec![1, 2, 3];
		let q = vec![1, 2, 3];
		let rootp = helper(p);
		let rootq = helper(q);
		let result = Solution1::is_same_tree(rootp, rootq);
		println!("{:?}", result);
	}

	#[test]
	fn test21() {
		let p = vec![1, 2];
		let q = vec![1, -1, 2];
		let rootp = helper(p);
		let rootq = helper(q);
		let result = Solution1::is_same_tree(rootp, rootq);
		println!("{:?}", result);
	}

	#[test]
	fn test31() {
		let p = vec![1, 2, 1];
		let q = vec![1, 1, 2];
		let rootp = helper(p);
		let rootq = helper(q);
		let result = Solution1::is_same_tree(rootp, rootq);
		println!("{:?}", result);
	}
}