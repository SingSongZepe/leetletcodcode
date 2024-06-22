

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32, expected: i32)  {
		let result = Solution::max_satisfied(customers, grumpy, minutes);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t1(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32, expected: i32)  {
		let result = Solution1::max_satisfied(customers, grumpy, minutes);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let customers = vec![1,0,1,2,1,1,7,5];
		let grumpy = vec![0,1,0,1,0,1,0,1];
		let minutes = 3;
		t(customers, grumpy, minutes, 16)
	}

	#[test]
	fn test2() {
		let customers = vec![1];
		let grumpy = vec![0];
		let minutes = 1;
		t(customers, grumpy, minutes, 1);
	}

	#[test]
	fn test11() {
		let customers = vec![1,0,1,2,1,1,7,5];
		let grumpy = vec![0,1,0,1,0,1,0,1];
		let minutes = 3;
		t1(customers, grumpy, minutes, 16)
	}

	#[test]
	fn test21() {
		let customers = vec![1];
		let grumpy = vec![0];
		let minutes = 1;
		t1(customers, grumpy, minutes, 1);
	}

	#[test]
	fn test31() {
		let customers = vec![4,10,10];
		let grumpy = vec![1,1,0];
		let minutes = 2;
		t1(customers, grumpy, minutes, 24);
	}

	#[test]
	fn test_windows() {
		let customers = vec![1,0,1,2,1,1,7,5];
		customers.windows(3).for_each(|w| println!("{:?}", w));
	}
}