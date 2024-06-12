

#[cfg(test)]
mod tests {
	use crate::*;

	fn t(n: i32, roads: Vec<Vec<i32>>, expected: i64) {
		let result = Solution::maximum_importance(n, roads);
		println!("{}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		// Input: n = 5, roads = [[0,1],[1,2],[2,3],[0,2],[1,3],[2,4]]
		// Output: 43
		let n = 5;
		let roads = vec![vec![0,1],vec![1,2],vec![2,3],vec![0,2],vec![1,3],vec![2,4]];
		let expected = 43;
		t(n, roads, expected);
	}

	#[test]
	fn test2() {
		// n = 5, roads = [[0,3],[2,4],[1,3]]
		// Output: 20
		let n = 5;
		let roads = vec![vec![0,3],vec![2,4],vec![1,3]];
		let expected = 20;
		t(n, roads, expected);
	}

}