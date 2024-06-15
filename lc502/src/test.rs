

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(arg: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>, expected: i32) {
		let result = Solution::find_maximized_capital(arg, w, profits, capital);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t2(arg: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>, expected: i32) {
		let result = Solution2::find_maximized_capital(arg, w, profits, capital);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}


	#[test]
	fn test1() {
		let k = 2;
		let w = 0;
		let profits = vec![1, 2, 3];
		let capital = vec![0, 1, 1];
		let expected = 4;
		t(k, w, profits, capital, expected);
	}

	#[test]
	fn test2() {
		let k = 3;
		let w = 0;
		let profits = vec![1, 2, 3];
		let capital = vec![0, 1, 2];
		let expected = 6;
		t(k, w, profits, capital, expected);
	}

	#[test]
	fn test3() {
		let k = 1;
		let w = 0;
		let profits = vec![1, 2, 3];
		let capital = vec![1, 1, 2];
		let expected = 0;
		t(k, w, profits, capital, expected)
	}

	#[test]
	fn test12() {
		let k = 2;
		let w = 0;
		let profits = vec![1, 2, 3];
		let capital = vec![0, 1, 1];
		let expected = 4;
		t2(k, w, profits, capital, expected);
	}

	#[test]
	fn test22() {
		let k = 3;
		let w = 0;
		let profits = vec![1, 2, 3];
		let capital = vec![0, 1, 2];
		let expected = 6;
		t2(k, w, profits, capital, expected);
	}

	#[test]
	fn test32() {
		let k = 1;
		let w = 0;
		let profits = vec![1, 2, 3];
		let capital = vec![1, 1, 2];
		let expected = 0;
		t2(k, w, profits, capital, expected)
	}

	#[test]
	fn test_scan() {
		let arr = [1, 3, 5, 7, 9, 11, 13, 15];
		let i = 2;
		let k = 10;

		let result = arr.iter().take_while(|&&x| x <= k).skip(i).collect::<Vec<_>>();
		let a = arr.get(i).filter(|&&x| x <= k).unwrap();

		for num in result {
			println!("{}", num);
		}
	}
}
