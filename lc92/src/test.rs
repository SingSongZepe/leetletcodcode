

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let v = helper(vec![1, 2, 3, 4, 5]);
		let left = 2;
		let right = 4;
		let result = Solution::reverse_between(v, left, right);
		print_list(&result);
	}

	// solution 1
	#[test]
	fn test2() {
		let v = helper(vec![5]);
		let left = 1;
		let right = 1;
		let result = Solution1::reverse_between(v, left, right);
		print_list(&result);
	}

	#[test]
	fn test11() {
		let v = helper(vec![1, 2, 3, 4, 5]);
		let left = 2;
		let right = 4;
		let result = Solution1::reverse_between(v, left, right);
		print_list(&result);
	}

	#[test]
	fn test21() {
		let v = helper(vec![5]);
		let left = 1;
		let right = 1;
		let result = Solution1::reverse_between(v, left, right);
		print_list(&result);
	}

	#[test]
	fn test_reverse() {
		let v = helper(vec![1, 2, 3]);
		let v_rev = Reverse::reverse_list(v);
		print_list(&v_rev);
	}

	#[test]
	fn test_take() {
		// fn use_take(mut v: Option<i32>) -> i32 {
		// 	let mut dummy_take = v;
		// 	let mut v_take = dummy_take.take();
		// 	v_take.unwrap() = 1;
		// 	dummy_take.unwrap()
		// 	// dummy_take.unwrap()
		// }
		// let v = 9;
		// println!("{:?}", use_take(Some(v)));
	}

	#[test]
	fn test_range() {
		let left = 2;
		let right = 4;
		let s = (0..left).chain(right..).take_while(|&x| x < 100).fold(0, |acc, x| acc + x);
		println!("{}", s);
	}
}