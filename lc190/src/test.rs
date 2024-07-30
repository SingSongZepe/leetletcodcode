

#[cfg(test)]
mod tests {
	use crate::*;

	fn t(n: u32, expected: u32) {
		let result = Solution::reverse_bits(n);
		println!("{:b}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let n: u32 = 0b00000010100101000001111010011100;
		t(n, 0b00111001011110000010100101000000);
	}
}