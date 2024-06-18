

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>, expected: i32) {
		let result = Solution::max_profit_assignment(difficulty, profit, worker);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t1(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>, expected: i32) {
		let result = Solution1::max_profit_assignment(difficulty, profit, worker);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t2(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>, expected: i32) {
		let result = Solution2::max_profit_assignment(difficulty, profit, worker);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let difficulty = vec![2,4,6,8,10];
		let profit = vec![10,20,30,40,50];
		let worker = vec![4,5,6,7];
		let expected = 100;
		t(difficulty, profit, worker, expected);
	}

	#[test]
	fn test2() {
		let difficulty = vec![85,47,57];
		let profit = vec![24,66,99];
		let worker = vec![40,25,25];
		let expected = 0;
		t(difficulty, profit, worker, expected);
	}

	#[test]
	fn test3() {
		let difficulty = vec![68,35,52,47,86];
		let profit = vec![67,17,1,81,3];
		let worker = vec![92,10,85,84,82];
		let expected = 324;
		t(difficulty, profit, worker, expected);
	}

	#[test]
	fn test4() {
		let difficulty = vec![5,50,92,21,24,70,17,63,30,53];
		let profit = vec![68,100,3,99,56,43,26,93,55,25];
		let worker = vec![96,3,55,30,11,58,68,36,26,1];
		let expected = 765;
		t(difficulty, profit, worker, expected);
	}

	#[test]
	fn test11() {
		let difficulty = vec![2,4,6,8,10];
		let profit = vec![10,20,30,40,50];
		let worker = vec![4,5,6,7];
		let expected = 100;
		t1(difficulty, profit, worker, expected);
	}

	#[test]
	fn test21() {
		let difficulty = vec![85,47,57];
		let profit = vec![24,66,99];
		let worker = vec![40,25,25];
		let expected = 0;
		t1(difficulty, profit, worker, expected);
	}

	#[test]
	fn test31() {
		let difficulty = vec![68,35,52,47,86];
		let profit = vec![67,17,1,81,3];
		let worker = vec![92,10,85,84,82];
		let expected = 324;
		t1(difficulty, profit, worker, expected);
	}


	#[test]
	fn test22() {
		let difficulty = vec![85,47,57];
		let profit = vec![24,66,99];
		let worker = vec![40,25,25];
		let expected = 0;
		t2(difficulty, profit, worker, expected);
	}

	#[test]
	fn test32() {
		let difficulty = vec![68,35,52,47,86];
		let profit = vec![67,17,1,81,3];
		let worker = vec![92,10,85,84,82];
		let expected = 324;
		t2(difficulty, profit, worker, expected);
	}

	#[test]
	fn test33() {
		let difficulty = vec![5,50,92,21,24,70,17,63,30,53];
		let profit = vec![68,100,3,99,56,43,26,93,55,25];
		let worker = vec![96,3,55,30,11,58,68,36,26,1];
		let expected = 765;
		t2(difficulty, profit, worker, expected);
	}
}