

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(rec1: Vec<i32>, rec2: Vec<i32>, expected: bool) {
		let result = Solution::is_rectangle_overlap(rec1, rec2);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let rec1 = vec![0, 0, 2, 2];
		let rec2 = vec![1, 1, 3, 3];
		t(rec1, rec2, true);
	}

	#[test]
	fn test2() {
		let rec1 = vec![0, 0, 1, 1];
		let rec2 = vec![1, 0, 2, 1];
		t(rec1, rec2, false);
	}

	#[test]
	fn test3() {
		let rec1 = vec![0, 0, 1, 1];
		let rec2 = vec![1, 1, 2, 2];
		t(rec1, rec2, false);
	}

}