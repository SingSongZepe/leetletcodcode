

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let path = "NES".to_string();
		let result = Solution::is_path_crossing(path);
		println!("{}", result);
	}

	#[test]
	fn test2() {
		let path = "NESWW".to_string();
		let result = Solution::is_path_crossing(path);
		println!("{}", result);
	}

	// solution 1
	#[test]
	fn test11() {
		let path = "NES".to_string();
		let result = Solution1::is_path_crossing(path);
		println!("{}", result);
	}

	#[test]
	fn test21() {
		let path = "NESWW".to_string();
		let result = Solution1::is_path_crossing(path);
		println!("{}", result);
	}
}