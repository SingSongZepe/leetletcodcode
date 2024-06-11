

#[cfg(test)]
mod tests {
	use crate::*;

	fn t(arr1: Vec<i32>, arr2: Vec<i32>, expected: Vec<i32>) {
		let result = Solution::relative_sort_array(arr1, arr2);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let arr1 = vec![2,3,1,3,2,4,6,7,9,2,19];
		let arr2 = vec![2,1,4,3,9,6];
		t(arr1, arr2, vec![2,2,2,1,4,3,3,9,6,7,19]);
	}

	#[test]
	fn test2() {
		let arr1 = vec![28,6,22,8,44,17];
		let arr2 = vec![22,28,8,6];
		t(arr1, arr2, vec![22,28,8,6,17,44]);
	}
}