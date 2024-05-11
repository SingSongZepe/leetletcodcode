

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let mut nums1 = vec![1, 2, 3, 0, 0, 0];
		let m = 3;
		let mut nums2 = vec![2, 5, 6];
		let n = 3;
		let result = Solution::merge(&mut nums1, m, &mut nums2, n);
		println!("{:?}", nums1);
	}

	#[test]
	fn test2() {
		let mut nums1 = vec![0];
		let m = 0;
		let mut nums2 = vec![1];
		let n = 1;
		let result = Solution::merge(&mut nums1, m, &mut nums2, n);
		println!("{:?}", nums1);
	}

}