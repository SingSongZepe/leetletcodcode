

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let quality = vec![10, 20, 5];
		let wage = vec![70,50,30];
		let k = 2;
		let result = Solution::mincost_to_hire_workers(quality, wage, k);
		println!("{:.5}", result);
	}

	#[test]
	fn test2() {
		let quality = vec![3,1,10,10,1];
		let wage = vec![4,8,2,2,7];
		let k = 3;
		let result = Solution::mincost_to_hire_workers(quality, wage, k);
		println!("{:.5}", result);
	}

	#[test]
	fn test3() {
		let quality = vec![32,43,66,9,94,57,25,44,99,19];
		let wage = vec![187,366,117,363,121,494,348,382,385,262];
		let k = 4;
		let result = Solution::mincost_to_hire_workers(quality, wage, k);
		println!("{:.5}", result);
	}

	#[test]
	fn test_formatted() {
		let result = 11113f32 / 7f32;
		println!("{:.5}", result);  // five numebers after decimal point
	}

}