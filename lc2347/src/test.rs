

#[cfg(test)]
mod tests {
	use crate::*;

	fn t(ranks: Vec<i32>, suits: Vec<char>, expected: String) {
		let result = Solution::best_hand(ranks, suits);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let ranks = vec![13,2,3,1,9];
		let suits = vec!['a','a','a','a','a'];
		t(ranks, suits, "Flush".to_string())
	}

	#[test]
	fn test2() {
		// ranks = [4,4,2,4,4], suits = ["d","a","a","b","c"]
		let ranks = vec![4,4,2,4,4];
		let suits = vec!['d','a','a','b','c'];
		t(ranks, suits, "Three of a Kind".to_string())
	}

	#[test]
	fn test3() {
		// ranks = [10,10,2,12,9], suits = ["a","b","c","a","d"]
		let ranks = vec![10,10,2,12,9];
		let suits = vec!['a','b','c','a','d'];
		t(ranks, suits, "Pair".to_string())
	}

	#[test]
	fn test4() {
		// ranks = [10,10,2,12,9], suits = ["a","b","c","a","d"]
		let ranks = vec![10,13,2,12,9];
		let suits = vec!['a','b','c','a','d'];
		t(ranks, suits, "High Card".to_string())
	}
}