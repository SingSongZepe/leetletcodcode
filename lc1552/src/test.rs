

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(position: Vec<i32>, m: i32, expected: i32)  {
		let result = Solution::max_distance(position, m);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let position = vec![1,2,3,4,7];
		let m = 3;
		let expected = 3;
		t(position, m, expected);
	}

	#[test]
	fn test2() {
		let position = vec![5,4,3,2,1,1000000000];
		let m = 2;
		let expected = 999999999;
		t(position, m, expected);
	}
}