

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(n: i32, expected: Vec<i32>)  {
		let result = Solution::even_odd_bit(n);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let n = 50;
		let expected = vec![1, 2];
		t(n, expected);
	}

	#[test]
	fn test2() {
		let n = 2;
		let expected = vec![0, 1];
		t(n, expected);
	}

}