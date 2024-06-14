

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(arg: Vec<Vec<i32>>, expected: i32) {
		let result = Solution::remove_stones(arg);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t1(arg: Vec<Vec<i32>>, expected: i32) {
		let result = Solution1::remove_stones(arg);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	#[test]
	fn test1() {
		let stones = vec![vec![0,0], vec![0,1], vec![1,0], vec![1,2], vec![2,1], vec![2,2]];
		t(stones, 5);
	}

	#[test]
	fn test2() {
		let stones = vec![vec![0,0], vec![0,2], vec![1,1], vec![2,0], vec![2,2]];
		t(stones, 3);
	}

	#[test]
	fn test3() {
		let stones = vec![vec![0, 0]];
		t(stones, 0);
	}

	#[test]
	fn test4() {
		// [[4,4],
		// [5,5],
		// [3,1],
		// [1,4],
		// [1,1],
		// [2,3],
		// [0,3],
		// [2,4],
		// [3,5]]
		let stones = vec![vec![4,4], vec![5,5], vec![3,1], vec![1,4], vec![1,1], vec![2,3], vec![0,3], vec![2,4], vec![3,5]];
		t(stones, 8);
	}

	#[test]
	fn test11() {
		let stones = vec![vec![0,0], vec![0,1], vec![1,0], vec![1,2], vec![2,1], vec![2,2]];
		t1(stones, 5);
	}

	#[test]
	fn test21() {
		let stones = vec![vec![0,0], vec![0,2], vec![1,1], vec![2,0], vec![2,2]];
		t1(stones, 3);
	}

	#[test]
	fn test31() {
		let stones = vec![vec![0, 0]];
		t1(stones, 0);
	}

	#[test]
	fn test41() {
		// [[4,4],
		// [5,5],
		// [3,1],
		// [1,4],
		// [1,1],
		// [2,3],
		// [0,3],
		// [2,4],
		// [3,5]]
		let stones = vec![vec![4,4], vec![5,5], vec![3,1], vec![1,4], vec![1,1], vec![2,3], vec![0,3], vec![2,4], vec![3,5]];
		t1(stones, 8);
	}


	#[test]
	fn test_hashset_from() {
		let mut hs = HashSet::from([1, 2, 3]);
		println!("{:?}", hs);
		println!("{:?}", hs.len());
	}

}