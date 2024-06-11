

#[cfg(test)]
mod tests {
	use crate::*;

	fn t(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>, expected: Vec<Vec<i32>>) {
		let result = Solution::merge_similar_items(items1, items2);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		// Input: items1 = [[1,1],[4,5],[3,8]], items2 = [[3,1],[1,5]]
		// Output: [[1,6],[3,9],[4,5]]
		let items1 = vec![vec![1,1], vec![4,5], vec![3,8]];
		let items2 = vec![vec![3,1], vec![1,5]];
		let expected = vec![vec![1,6], vec![3,9], vec![4,5]];
		t(items1, items2, expected);
	}

	#[test]
	fn test2() {
		// Input: items1 = [[1,1],[3,2],[2,3]], items2 = [[2,1],[3,2],[1,3]]
		// Output: [[1,4],[2,4],[3,4]]
		let items1 = vec![vec![1,1], vec![3,2], vec![2,3]];
		let items2 = vec![vec![2,1], vec![3,2], vec![1,3]];
		let expected = vec![vec![1,4], vec![2,4], vec![3,4]];
		t(items1, items2, expected);
	}

	#[test]
	fn test3() {
		// Input: items1 = [[1,3],[2,2]], items2 = [[7,1],[2,2],[1,4]]
		// Output: [[1,7],[2,4],[7,1]]
		let items1 = vec![vec![1,3], vec![2,2]];
		let items2 = vec![vec![7,1], vec![2,2], vec![1,4]];
		let expected = vec![vec![1,7], vec![2,4], vec![7,1]];
		t(items1, items2, expected);
	}

}




