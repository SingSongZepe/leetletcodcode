

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let prices = vec![1, 2, 2];
		let money = 3;
		let result = Solution::buy_choco(prices, money);
		println!("{}", result);
	}

	#[test]
	fn test2() {
		let prices = vec![3, 2, 3];
		let money = 3;
		let result = Solution::buy_choco(prices, money);
		println!("{}", result);
	}

	// Solution 1
	#[test]
	fn test11() {
		let prices = vec![1, 2, 2];
		let money = 3;
		let result = Solution1::buy_choco(prices, money);
		println!("{}", result);
	}

	#[test]
	fn test21() {
		let prices = vec![3, 2, 3];
		let money = 3;
		let result = Solution1::buy_choco(prices, money);
		println!("{}", result);
	}

}