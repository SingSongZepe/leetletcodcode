

#[cfg(test)]
mod tests {
	use crate::*;

	fn t(names: Vec<String>, times: Vec<String>, expected: Vec<String>) {
		let result = Solution::alert_names(names, times);
		println!("{:?}", result);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test1() {
		let keyName = helper(vec!["daniel", "daniel","daniel","luis","luis","luis","luis"]);
		let keyTime = helper(vec!["10:00","10:40","11:00","09:00","11:00","13:00","15:00"]);
		t(keyName, keyTime, helper(vec!["daniel"]));
	}

	#[test]
	fn test2() {
		let keyName = helper(vec!["alice","alice","alice","bob","bob","bob","bob"]);
		let keyTime = helper(vec!["12:01","12:00","18:00","21:00","21:20","21:30","23:00"]);
		t(keyName, keyTime, helper(vec!["bob"]));
	}

	#[test]
	fn test_slice() {
		let keyName = helper(vec!["alice", "alice", "alice", "bob", "bob", "bob"]);
		let keyTime = helper(vec!["12:05", "12:06", "12:07", "12:08", "12:09", "12:10"]);
		// println!("{}", keyName[-1])
	}
}