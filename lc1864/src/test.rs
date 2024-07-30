

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let s = "111000".to_string();
		let result = Solution::min_swaps(s);
		println!("{}", result);
	}

	#[test]
	fn test2() {
		let s = "010".to_string();
		let result = Solution::min_swaps(s);
		println!("{}", result);
	}

	#[test]
	fn test3() {
		let s = "1110".to_string();
		let result = Solution::min_swaps(s);
		println!("{}", result);
	}

	#[test]
	fn test4() {
		let s = "00011110110110000000000110110101011101111011111101010010010000000000000001101101010010001011110000001101111111110000110101101101001011000011111011101101100110011111110001100110001110000000001100010111110100111001001111100001000110101111010011001".to_string();
		let result = Solution::min_swaps(s);
		println!("{}", result);
	}


	#[test]
	fn test11() {
		let s = "111000".to_string();
		let result = Solution1::min_swaps(s);
		println!("{}", result);
	}

	#[test]
	fn test21() {
		let s = "010".to_string();
		let result = Solution1::min_swaps(s);
		println!("{}", result);
	}

	#[test]
	fn test31() {
		let s = "1110".to_string();
		let result = Solution1::min_swaps(s);
		println!("{}", result);
	}

	#[test]
	fn test41() {
		let s = "00011110110110000000000110110101011101111011111101010010010000000000000001101101010010001011110000001101111111110000110101101101001011000011111011101101100110011111110001100110001110000000001100010111110100111001001111100001000110101111010011001".to_string();
		let result = Solution1::min_swaps(s);
		println!("{}", result);
	}
}