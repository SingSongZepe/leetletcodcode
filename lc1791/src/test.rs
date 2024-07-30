

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(edges: Vec<Vec<i32>>, expected: i32) {
		let result = Solution::find_center(edges);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let v = vec![vec![1,2],vec![2,3],vec![4,5]];
		let expected = 2;
		t(v, expected);
	}

	#[test]
	fn test2() {
		let v = vec![vec![1,2],vec![5,1],vec![1,3],vec![1,4]];
		let expected = 1;
		t(v, expected);
	}

}