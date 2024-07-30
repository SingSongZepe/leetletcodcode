

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(bloom_day: Vec<i32>, m: i32, k: i32, expected: i32) {
		let result = Solution::min_days(bloom_day, m, k);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t1(bloom_day: Vec<i32>, m: i32, k: i32, expected: i32) {
		let result = Solution1::min_days(bloom_day, m, k);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t2(bloom_day: Vec<i32>, m: i32, k: i32, expected: i32) {
		let result = Solution2::min_days(bloom_day, m, k);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	#[test]
	fn test1() {
		let bloom_day = vec![1, 10, 3, 10, 2];
		let m = 3;
		let k = 1;
		let expected = 3;
		t(bloom_day, m, k, expected);
	}

	#[test]
	fn test2() {
		let bloom_day = vec![1, 10, 3, 10, 2];
		let m = 3;
		let k = 2;
		let expected = -1;
		t(bloom_day, m, k, expected);
	}

	#[test]
	fn test3() {
		let bloom_day = vec![5,37,55,92,22,52,31,62,99,64,92,53,34,84,93,50,28];
		let m = 8;
		let k = 2;
		let expected = 93;
		t(bloom_day, m, k, expected);
	}

	#[test]
	fn test11() {
		let bloom_day = vec![1, 10, 3, 10, 2];
		let m = 3;
		let k = 1;
		let expected = 3;
		t1(bloom_day, m, k, expected);
	}

	#[test]
	fn test21() {
		let bloom_day = vec![1, 10, 3, 10, 2];
		let m = 3;
		let k = 2;
		let expected = -1;
		t1(bloom_day, m, k, expected);
	}

	#[test]
	fn test31() {
		let bloom_day = vec![5,37,55,92,22,52,31,62,99,64,92,53,34,84,93,50,28];
		let m = 8;
		let k = 2;
		let expected = 93;
		t1(bloom_day, m, k, expected);
	}

	#[test]
	fn test12() {
		let bloom_day = vec![1, 10, 3, 10, 2];
		let m = 3;
		let k = 1;
		let expected = 3;
		t2(bloom_day, m, k, expected);
	}

	#[test]
	fn test22() {
		let bloom_day = vec![1, 10, 3, 10, 2];
		let m = 3;
		let k = 2;
		let expected = -1;
		t2(bloom_day, m, k, expected);
	}

	#[test]
	fn test32() {
		let bloom_day = vec![5,37,55,92,22,52,31,62,99,64,92,53,34,84,93,50,28];
		let m = 8;
		let k = 2;
		let expected = 93;
		t2(bloom_day, m, k, expected);
	}

	#[test]
	fn test42() {
		let bloom_day = vec![7,7,7,7,12,7,7];
		let m = 2;
		let k = 3;
		let expected = 12;
		t2(bloom_day, m, k, expected);
	}



	#[test]
	fn test_windows() {
		let bloom_day = vec![1, 10, 3, 10, 2];
		bloom_day.iter().skip(1).collect::<Vec<_>>().chunks_exact(2).for_each(|w| {
			println!("{:?}", w);
		});
	}

	#[test]
	fn test_i32wrapper() {
		let a = ReverseWrapper(10);
		let b = ReverseWrapper(5);
		let mut bh = BinaryHeap::new();
		bh.push(a);
		bh.push(b);
		while let Some(ReverseWrapper(x)) = bh.pop() {
			println!("{}", x);
		}
	}
}