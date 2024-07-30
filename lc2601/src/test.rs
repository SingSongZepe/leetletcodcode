

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let nums = vec![4, 9, 6, 10];
		let result = Solution::prime_sub_operation(nums);
		println!("{:?}", result);
	}

	#[test]
	fn test2() {
		let nums = vec![6, 8, 11, 12];
		let result = Solution::prime_sub_operation(nums);
		println!("{:?}", result);
	}

	#[test]
	fn test3() {
		let nums = vec![5, 8, 3];
		let result = Solution::prime_sub_operation(nums);
		println!("{:?}", result);
	}

	#[test]
	fn test4() {
		let nums = vec![2, 2];
		let result = Solution::prime_sub_operation(nums);
		println!("{:?}", result);
	}

	#[test]
	fn test5() {
		let nums = vec![998, 2];
		let result = Solution::prime_sub_operation(nums);
		println!("{:?}", result);
	}

	#[test]
	fn test_find_largest_prime_less_than_n() {
		println!("{}", Solution::find_largest_prime_less_than_n(10).unwrap());
		println!("{}", Solution::find_largest_prime_less_than_n(20).unwrap());
		println!("{}", Solution::find_largest_prime_less_than_n(300).unwrap());
		println!("{}", Solution::find_largest_prime_less_than_n(456).unwrap());
	}
}