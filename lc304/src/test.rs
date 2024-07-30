

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(arg: Vec<Vec<i32>>, ranges: Vec<Vec<i32>>, expected: Vec<i32>) {
		let nm = NumMatrix::new(arg);
		let mut result = vec![];
		for r in ranges {
			let v = nm.sum_region(r[0], r[1], r[2], r[3]);
			result.push(v);
			println!("{:?}", v);
		}
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		// [[3, 0, 1, 4, 2], [5, 6, 3, 2, 1], [1, 2, 0, 1, 5], [4, 1, 0, 1, 7], [1, 0, 3, 0, 5]]
		//  [2, 1, 4, 3], [1, 1, 2, 2], [1, 2, 2, 4]
		let matrix = vec![vec![3, 0, 1, 4, 2], vec![5, 6, 3, 2, 1], vec![1, 2, 0, 1, 5], vec![4, 1, 0, 1, 7], vec![1, 0, 3, 0, 5]];
		let ranges = vec![vec![2, 1, 4, 3], vec![1, 1, 2, 2], vec![1, 2, 2, 4]];
		let expected = vec![8, 11, 12];
		t(matrix, ranges, expected);
	}

}