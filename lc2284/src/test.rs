

#[cfg(test)]
mod tests {
	use crate::*;

	fn t(messages: Vec<String>, senders: Vec<String>, expected: String) {
		let result = Solution::largest_word_count(messages, senders);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	fn t1(messages: Vec<String>, senders: Vec<String>, expected: String) {
		let result = Solution1::largest_word_count(messages, senders);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}

	#[test]
	fn test1() {
		let messages = helper(vec!["Hello userTwooo","Hi userThree","Wonderful day Alice","Nice day userThree"]);
		let senders = helper(vec!["Alice","userTwo","userThree","Alice"]);
		t(messages, senders, "Alice".to_string())
	}

	#[test]
	fn test2() {
		// Input: messages = ["How is leetcode for everyone","Leetcode is useful for practice"], senders = ["Bob","Charlie"]
		// Output: "Charlie"
		let messages = helper(vec!["How is leetcode for everyone","Leetcode is useful for practice"]);
		let senders = helper(vec!["Bob","Charlie"]);
		t(messages, senders, "Charlie".to_string())
	}

	#[test]
	fn test3() {
		// Input: messages = ["a","b"], senders = ["Bob","Charlie"]
		// Output: "Charlie"
		let messages = helper(vec!["a","b"]);
		let senders = helper(vec!["Bob","Charlie"]);
		t(messages, senders, "Bob".to_string())
	}

	#[test]
	fn test11() {
		let messages = helper(vec!["Hello userTwooo","Hi userThree","Wonderful day Alice","Nice day userThree"]);
		let senders = helper(vec!["Alice","userTwo","userThree","Alice"]);
		t1(messages, senders, "Alice".to_string())
	}

	#[test]
	fn test21() {
		// Input: messages = ["How is leetcode for everyone","Leetcode is useful for practice"], senders = ["Bob","Charlie"]
		// Output: "Charlie"
		let messages = helper(vec!["How is leetcode for everyone","Leetcode is useful for practice"]);
		let senders = helper(vec!["Bob","Charlie"]);
		t1(messages, senders, "Charlie".to_string())
	}

	#[test]
	fn test31() {
		// Input: messages = ["a","b"], senders = ["Bob","Charlie"]
		// Output: "Charlie"
		let messages = helper(vec!["a","b"]);
		let senders = helper(vec!["Bob","Charlie"]);
		t1(messages, senders, "Charlie".to_string())
	}

	#[test]
	fn test41() {
		// Input: messages = ["a","b"], senders = ["Bob","Charlie"]
		// Output: "Charlie"
		let messages = helper(vec!["tP x M VC h lmD","D X XF w V","sh m Pgl","pN pa","C SL m G Pn v","K z UL B W ee","Yf yo n V U Za f np","j J sk f qr e v t","L Q cJ c J Z jp E","Be a aO","nI c Gb k Y C QS N","Yi Bts","gp No g s VR","py A S sNf","ZS H Bi De dj dsh","ep MA KI Q Ou"]);
		let senders = helper(vec!["OXlq","IFGaW","XQPeWJRszU","Gb","HArIr","Gb","FnZd","FnZd","HArIr","OXlq","IFGaW","XQPeWJRszU","EMoUs","Gb","EMoUs","EMoUs"]);
		t1(messages, senders, "FnZd".to_string())
	}

	#[test]
	fn test_min_string() {
		let user1 = "Alice".to_string();
		let user2 = "Chase".to_string();
		if user1 > user2 {
			println!("{}", user1);
		} else {
			println!("{}", user2)
		}
		let user3 = "Bob".to_string();
		let userlist = vec![user1, user2, user3];
		let min_user = userlist.iter().min().unwrap().to_string();
		println!("{}", min_user);
	}

}