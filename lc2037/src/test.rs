

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(seats: Vec<i32>, students: Vec<i32>, expected: i32) {
		let result = Solution::min_moves_to_seat(seats, students);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	#[test]
	fn test1() {
		let seats = vec![3, 1, 5];
		let students = vec![2, 7, 4];
		t(seats, students, 4);
	}
	
	#[test]
	fn test2() {
		let seats = vec![4, 1, 5, 9];
		let students = vec![1, 3, 2, 6];
		t(seats, students, 7);
	}

	#[test]
	fn test3() {
		let seats = vec![2,2,6,6];
		let students = vec![1,3,2,6];
		t(seats, students, 4);
	}

}