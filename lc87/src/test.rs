

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let s1 = "great".to_string();
		let s2 = "rgeat".to_string();
		println!("{:?}", Solution::is_scramble(s1, s2));
	}

	#[test]
	fn test2() {
		let s1 = "abcde".to_string();
		let s2 = "caebd".to_string();
		println!("{:?}", Solution::is_scramble(s1, s2));
	}

	#[test]
	fn test3() {
		let s1 = "a".to_string();
		let s2 = "a".to_string();
		println!("{:?}", Solution::is_scramble(s1, s2));
	}

	#[test]
	fn test4() {
		let s1 = "abca".to_string();
		let s2 = "caba".to_string();
		println!("{:?}", Solution::is_scramble(s1, s2));
	}


	// test why any string that length <= 3 is scramble
	// but when string comes to length > 3, it is maybe not scramble
	#[test]
	fn test5() {
		let s1 = "abc".to_string();
		let s2 = "acb".to_string();
		let s3 = "bac".to_string();
		let s4 = "bca".to_string();
		let s5 = "cab".to_string();
		let s6 = "cba".to_string();
		println!("{:?}", Solution::is_scramble(s1.clone(), s2));
		println!("{:?}", Solution::is_scramble(s1.clone(), s3));
		println!("{:?}", Solution::is_scramble(s1.clone(), s4));
		println!("{:?}", Solution::is_scramble(s1.clone(), s5));
		println!("{:?}", Solution::is_scramble(s1.clone(), s6));
	}


	#[test]
	fn test_extend_u8() {
		let s1 = "abcde".to_string();
		let s2 = "caebd".to_string();
		let u1 = s1.as_bytes();
		let u2 = s2.as_bytes();

		let s3 = "abcde";
		let s4 = "caebd";

	}

}