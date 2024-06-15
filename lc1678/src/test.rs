

#[cfg(test)]
mod tests {
	use crate::*;
	
	fn t(command: String, expected: String) {
		let result = Solution::interpret(command);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t1(command: String, expected: String) {
		let result = Solution1::interpret(command);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t2(command: String, expected: String) {
		let result = Solution2::interpret(command);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}


	#[test]
	fn test1() {
		let command = "G()(al)".to_string();
		t(command, "Goal".to_string())
	}

	#[test]
	fn test2() {
		let command = "G()()()()(al)".to_string();
		t(command, "Gooooal".to_string())
	}

	#[test]
	fn test3() {
		let command = "(al)G(al)()()G".to_string();
		t(command, "alGalooG".to_string())
	}

	#[test]
	fn test11() {
		let command = "G()(al)".to_string();
		t1(command, "Goal".to_string())
	}

	#[test]
	fn test21() {
		let command = "G()()()()(al)".to_string();
		t1(command, "Gooooal".to_string())
	}

	#[test]
	fn test31() {
		let command = "(al)G(al)()()G".to_string();
		t1(command, "alGalooG".to_string())
	}


	#[test]
	fn test12() {
		let command = "G()(al)".to_string();
		t2(command, "Goal".to_string())
	}

	#[test]
	fn test22() {
		let command = "G()()()()(al)".to_string();
		t2(command, "Gooooal".to_string())
	}

	#[test]
	fn test32() {
		let command = "(al)G(al)()()G".to_string();
		t2(command, "alGalooG".to_string())
	}
}