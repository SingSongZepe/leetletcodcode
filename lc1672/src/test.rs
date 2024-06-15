

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(arg: Vec<Vec<i32>>, expected: i32) {
		let result = Solution::maximum_wealth(arg);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		// [[1,2,3],[3,2,1]]
		let account = vec![vec![1,2,3], vec![3,2,1]];
		let expected = 6;
		t(account, expected);
	}

	#[test]
	fn test2() {
		// [[1,5],[7,3],[3,5]]
		let account = vec![vec![1,5], vec![7,3], vec![3,5]];
		let expected = 10;
		t(account, expected);
	}

	#[test]
	fn test3() {
		// [[2,8,7],[7,1,3],[1,9,5]]
		let account = vec![vec![2,8,7], vec![7,1,3], vec![1,9,5]];
		let expected = 17;
		t(account, expected);
	}

}