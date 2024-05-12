

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let v = helper(vec![1, 2, 3]);
	}


	#[test]
	fn test_reverse() {
		let v = helper(vec![1, 2, 3]);
		let v_rev = Reverse::reverse_list(v);
		print_list(&v_rev);
	}
}