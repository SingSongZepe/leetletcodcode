

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let numOnes = 3;
		let numZeros = 2;
		let numNegOnes = 0;
		let k = 2;
		let result = Solution::k_items_with_maximum_sum(numOnes, numZeros, numNegOnes, k);
		println!("{}", result); // expected output: 2
	}

	#[test]
	fn test2() {
		let numOnes = 3;
		let numZeros = 2;
		let numNegOnes = 0;
		let k = 4;
		let result = Solution::k_items_with_maximum_sum(numOnes, numZeros, numNegOnes, k);
		println!("{}", result); // expected output: 3
	}

	#[test]
	fn test3() {
		let numOnes = 3;
		let numZeros = 2;
		let numNegOnes = 10;
		let k = 7;
		let result = Solution::k_items_with_maximum_sum(numOnes, numZeros, numNegOnes, k);
		println!("{}", result); // expected output: 3
	}
}