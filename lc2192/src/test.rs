

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(n: i32, edges: Vec<Vec<i32>>, expected: Vec<Vec<i32>>) {
		let result = Solution::get_ancestors(n, edges);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		// Input: n = 5, edgeList = [[0,1],[0,2],[0,3],[0,4],[1,2],[1,3],[1,4],[2,3],[2,4],[3,4]]
		// Output: [[],[0],[0,1],[0,1,2],[0,1,2,3]]
		let n = 5;
		let edges = vec![vec![0,1],vec![0,2],vec![0,3],vec![0,4],vec![1,2],vec![1,3],vec![1,4],vec![2,3],vec![2,4],vec![3,4]];
		let expected = vec![vec![],vec![0],vec![0,1,2],vec![0,1,2,3]];
		t(n, edges, expected);
	}

	#[test]
	fn test2() {
		// Input: n = 8, edgeList = [[0,3],[0,4],[1,3],[2,4],[2,7],[3,5],[3,6],[3,7],[4,6]]
		// Output: [[],[],[],[0,1],[0,2],[0,1,3],[0,1,2,3,4],[0,1,2,3]]
		let n = 8;
		let edges = vec![vec![0,3],vec![0,4],vec![1,3],vec![2,4],vec![2,7],vec![3,5],vec![3,6],vec![3,7],vec![4,6]];
		let expected = vec![vec![],vec![0,1,2,3,4],vec![0,1,2,3,4,7],vec![0,1,2,3,4,5,6,7],vec![0,1,2,3,4,5,6,7],vec![0,1,2,3,4,5,6,7]];
		t(n, edges, expected);
	}
}