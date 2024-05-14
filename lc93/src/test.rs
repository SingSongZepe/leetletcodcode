

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let s = "25525511135".to_string();
		let result = Solution::restore_ip_addresses(s);
		println!("{:?}", result);
	}

	#[test]
	fn test2() {
		let s = "0000".to_string();
		let result = Solution::restore_ip_addresses(s);
		println!("{:?}", result);
	}

	#[test]
	fn test3() {
		let s = "101023".to_string();
		let result = Solution::restore_ip_addresses(s);
		println!("{:?}", result);
	}
}