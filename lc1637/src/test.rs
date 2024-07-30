

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		// [[8,7],[9,9],[7,4],[9,7]]
		let points = vec![
			vec![8, 7],
			vec![9, 9],
			vec![7, 4],
			vec![9, 7]
		];
		let result = Solution::max_width_of_vertical_area(points);
		println!("{}", result); // expected 1
	}

	#[test]
	fn test2() {
		// [[3,1],[9,0],[1,0],[1,4],[5,3],[8,8]]
		let points = vec![
			vec![3, 1],
			vec![9, 0],
			vec![1, 0],
			vec![1, 4],
			vec![5, 3],
			vec![8, 8]
		];
		let result = Solution::max_width_of_vertical_area(points);
		println!("{}", result); // expected 3
	}

	// Solution 1
	#[test]
	fn test11() {
		// [[8,7],[9,9],[7,4],[9,7]]
		let points = vec![
			vec![8, 7],
			vec![9, 9],
			vec![7, 4],
			vec![9, 7]
		];
		let result = Solution1::max_width_of_vertical_area(points);
		println!("{}", result); // expected 1
	}

	#[test]
	fn test21() {
		// [[3,1],[9,0],[1,0],[1,4],[5,3],[8,8]]
		let points = vec![
			vec![3, 1],
			vec![9, 0],
			vec![1, 0],
			vec![1, 4],
			vec![5, 3],
			vec![8, 8]
		];
		let result = Solution1::max_width_of_vertical_area(points);
		println!("{}", result); // expected 3
	}

	// Solution 2
	#[test]
	fn test12() {
		// [[8,7],[9,9],[7,4],[9,7]]
		let points = vec![
			vec![8, 7],
			vec![9, 9],
			vec![7, 4],
			vec![9, 7]
		];
		let result = Solution2::max_width_of_vertical_area(points);
		println!("{}", result); // expected 1
	}

	#[test]
	fn test22() {
		// [[3,1],[9,0],[1,0],[1,4],[5,3],[8,8]]
		let points = vec![
			vec![3, 1],
			vec![9, 0],
			vec![1, 0],
			vec![1, 4],
			vec![5, 3],
			vec![8, 8]
		];
		let result = Solution2::max_width_of_vertical_area(points);
		println!("{}", result); // expected 3
	}

	#[test]
	fn test_sort_vector() {
		let mut v = vec![
			vec![3, 1],
			vec![9, 0],
			vec![1, 0],
			vec![1, 4],
			vec![5, 3],
			vec![8, 8]
		];
		v.sort();
		println!("{:?}", v);
	}
}