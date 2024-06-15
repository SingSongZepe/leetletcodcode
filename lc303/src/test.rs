

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(arg: Vec<i32>, ranges: Vec<Vec<i32>>, expected: Vec<i32>) {
		// Your NumArray object will be instantiated and called as such:
		let obj = NumArray::new(arg);
		let mut result = vec![];

		for r in ranges {
			let v = obj.sum_range(r[0], r[1]);
			result.push(v);
			println!("{}", v);
		}
		assert_eq!(result, expected);
	}

	fn t1(arg: Vec<i32>, ranges: Vec<Vec<i32>>, expected: Vec<i32>) {
		// Your NumArray object will be instantiated and called as such:
		let obj = NumArray1::new(arg);
		let mut result = vec![];

		for r in ranges {
			let v = obj.sum_range(r[0], r[1]);
			result.push(v);
			println!("{}", v);
		}
		assert_eq!(result, expected);
	}

	fn t2(arg: Vec<i32>, ranges: Vec<Vec<i32>>, expected: Vec<i32>) {
		// Your NumArray object will be instantiated and called as such:
		let obj = NumArray2::new(arg);
		let mut result = vec![];

		for r in ranges {
			let v = obj.sum_range(r[0], r[1]);
			result.push(v);
			println!("{}", v);
		}
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let nums = vec![-2, 0, 3, -5, 2, -1];
		let ranges = vec![vec![0, 2], vec![2, 5], vec![0, 5]];
		let expected = vec![1, -1, -3];
		t(nums, ranges, expected)
	}

	#[test]
	fn test11() {
		let nums = vec![-2, 0, 3, -5, 2, -1];
		let ranges = vec![vec![0, 2], vec![2, 5], vec![0, 5]];
		let expected = vec![1, -1, -3];
		t1(nums, ranges, expected)
	}


	#[test]
	fn test12() {
		let nums = vec![-2, 0, 3, -5, 2, -1];
		let ranges = vec![vec![0, 2], vec![2, 5], vec![0, 5]];
		let expected = vec![1, -1, -3];
		t2(nums, ranges, expected)
	}

}