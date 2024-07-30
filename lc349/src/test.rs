

#[cfg(test)]
mod tests {
	use crate::*;

	fn t(nums1: Vec<i32>, nums2: Vec<i32>, expected: Vec<i32>) {
		let result = Solution::intersection(nums1, nums2);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let nums1 = vec![1, 2, 2, 1];
		let nums2 = vec![2, 2];
		let expected = vec![2];
		t(nums1, nums2, expected);
	}

	#[test]
	fn test2() {
		let nums1 = vec![4, 9, 5];
		let nums2 = vec![9, 4, 9, 8, 4];
		let expected = vec![4, 9];
		t(nums1, nums2, expected);
	}

}