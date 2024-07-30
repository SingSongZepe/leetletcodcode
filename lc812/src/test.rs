

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(points: Vec<Vec<i32>>, expected: f64) {
		let result = Solution::largest_triangle_area(points);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		// points = [[0,0],[0,1],[1,0],[0,2],[2,0]]
		let points = vec![vec![0,0],vec![0,1],vec![1,0],vec![0,2],vec![2,0]];
		let expected = 2.0;
		t(points, expected);
	}

	#[test]
	fn test2() {
		// points = [[1,0],[0,0],[0,1]]
		let points = vec![vec![1, 0], vec![0, 1], vec![0, 0]];
		let expected = 0.5;
		t(points, expected);
	}
}