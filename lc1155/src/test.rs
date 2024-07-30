

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let n = 1;
		let k = 6;
		let target = 3;
		let result = Solution::num_rolls_to_target(n, k, target);
		println!("{}", result);
	}

	#[test]
	fn test2() {
		let n = 2;
		let k = 6;
		let target = 7;
		let result = Solution::num_rolls_to_target(n, k, target);
		println!("{}", result);
	}

	#[test]
	fn test3() {
		let n = 30;
		let k = 30;
		let target = 500;
		let result = Solution::num_rolls_to_target(n, k, target);
		println!("{}", result);
	}


	#[test]
	fn test4() {
		let n = 3;
		let k = 3;
		let target = 5;
		let result = Solution::num_rolls_to_target(n, k, target);
		println!("{}", result);
	}

	#[test]
	fn test11() {
		let n = 1;
		let k = 6;
		let target = 3;
		let result = Solution1::num_rolls_to_target(n, k, target);
		println!("{}", result);
	}

	#[test]
	fn test21() {
		let n = 2;
		let k = 6;
		let target = 7;
		let result = Solution1::num_rolls_to_target(n, k, target);
		println!("{}", result);
	}

	#[test]
	fn test31() {
		let n = 30;
		let k = 30;
		let target = 500;
		let result = Solution1::num_rolls_to_target(n, k, target);
		println!("{}", result);
	}


	#[test]
	fn test41() {
		let n = 3;
		let k = 3;
		let target = 5;
		let result = Solution1::num_rolls_to_target(n, k, target);
		println!("{}", result);
	}
}