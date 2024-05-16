

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		// [[1,null,2,null,3],[1,null,3,2],[2,1,3],[3,1,null,null,2],[3,2,null,1]]
		// make the vec vec to Vec<Vec<i32>>, use -1 represent null
		let n = 3;
		let result = Solution::generate_trees(n);

	}


	#[test]
	fn test2() {
		// [[1,null,2,null,3],[1,null,3,2],[2,1,3],[3,1,null,null,2],[3,2,null,1]]
		// make the vec vec to Vec<Vec<i32>>, use -1 represent null
		let n = 1;
		let result = Solution::generate_trees(n);

	}
}