

#[cfg(test)]
mod tests {
	use crate::*;

	fn t(start: i32, goal: i32) {
		let result = Solution::min_bit_flips(start, goal);
		println!("{}", result);
	}

	#[test]
	fn test1() {
		let start = 10;
		let goal = 7;
		t(start, goal);
	}

	#[test]
	fn test2() {
		let start = 3;
		let goal = 4;
		t(start, goal);
	}

}