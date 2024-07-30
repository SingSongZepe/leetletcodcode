

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(encoded: Vec<i32>, first: i32, expected: Vec<i32>) {
		let result = Solution::decode(encoded, first);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t1(encoded: Vec<i32>, first: i32, expected: Vec<i32>) {
		let result = Solution1::decode(encoded, first);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	#[test]
	fn test1() {
		let encoded = vec![1,2,3];
		let first = 1;
		let expected = vec![1, 0, 2, 1];
		t(encoded, first, expected);
	}

	#[test]
	fn test2() {
		let encoded = vec![6,2,7,3];
		let first = 4;
		let expected = vec![4,2,0,7,4];
		t(encoded, first, expected);
	}

	#[test]
	fn test11() {
		let encoded = vec![1,2,3];
		let first = 1;
		let expected = vec![1, 0, 2, 1];
		t1(encoded, first, expected);
	}

	#[test]
	fn test21() {
		let encoded = vec![6,2,7,3];
		let first = 4;
		let expected = vec![4,2,0,7,4];
		t1(encoded, first, expected);
	}

}