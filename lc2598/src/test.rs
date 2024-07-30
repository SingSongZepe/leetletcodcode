

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let nums = vec![1,-10,7,13,6,8];
		let value = 5;
		let result = Solution::find_smallest_integer(nums, value);
		println!("{}", result); // output: 4
	}

	#[test]
	fn test2() {
		let nums = vec![1,-10,7,13,6,8];
		let value = 7;
		let result = Solution::find_smallest_integer(nums, value);
		println!("{}", result); // output: 2
	}


	#[test]
	fn test_mod() {
		let nums = vec![1,-10,7,13,6,8];
		for &n in &nums {
			println!("{} % 5 = {}", n, n % 5);
		}
	}
}