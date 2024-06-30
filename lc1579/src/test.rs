

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(n: i32, edges: Vec<Vec<i32>>, expected: i32){
		let result = Solution::max_num_edges_to_remove(n, edges);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		// n = 4, edges = [[3,1,2],[3,2,3],[1,1,3],[1,2,4],[1,1,2],[2,3,4]]
		// expected = 2
		let n = 4;
		let edges = vec![vec![3,1,2],vec![3,2,3],vec![1,1,3],vec![1,2,4],vec![1,1,2],vec![2,3,4]];
		let expected = 2;
		t(n, edges, expected);
	}

	#[test]
	fn test2() {
		// n = 4, edges = [[3,1,2],[3,2,3],[1,1,4],[2,1,4]]
		// expected = 0
		let n = 4;
		let edges = vec![vec![3,1,2],vec![3,2,3],vec![1,1,4],vec![2,1,4]];
		let expected = 0;
		t(n, edges, expected);
	}

	#[test]
	fn test3() {
		// n = 4, edges = [[3,2,3],[1,1,2],[2,3,4]]
		// expected = -1
		let n = 4;
		let edges = vec![vec![3,2,3],vec![1,1,2],vec![2,3,4]];
		let expected = -1;
		t(n, edges, expected);
	}
}