mod test;

struct Solution;
use std::collections::HashMap;

impl Solution {
	pub fn is_scramble(s1: String, s2: String) -> bool {
		let mut memo = HashMap::<(&str, &str), bool>::new();
		let s1 = s1.as_str();
		let s2 = s2.as_str();
		Solution::it(s1, s2, &mut memo)
	}

	fn it<'a>(s1: &'a str, s2: &'a str, memo: &mut HashMap<(&'a str, &'a str), bool>) -> bool {
		if s1.len() != s2.len() {
			return false;
		}
		if s1 == s2 {
			return true;
		}
		if !Self::is_anagram(s1, s2) {
			return false;
		}
		if s1.len() <= 3 { /// ！！！！any smaller than 3 is scramble
			return true;
		}

		if memo.contains_key(&(s1, s2)) {
			return *memo.get(&(s1, s2)).unwrap();
		}

		let n = s1.len();

		for i in 1..n {
			if Solution::it(&s1[..i], &s2[..i], memo) && Solution::it(&s1[i..], &s2[i..], memo)
				|| Solution::it(&s1[..i], &s2[n - i..], memo) && Solution::it(&s1[i..], &s2[..n - i], memo) {
				memo.insert((s1, s2), true);
				return true;
			}
		}

		memo.insert((s1, s2), false);
		false
	}
	fn is_anagram(s1: &str, s2: &str) -> bool {
		let mut char_count = [0; 26];
		for &c in s1.as_bytes() {
			char_count[(c - b'a') as usize] += 1;
		}
		for &c in s2.as_bytes() {
			char_count[(c - b'a') as usize] -= 1;
		}
		char_count.iter().all(|&x| x == 0)
	}
}


fn main() {
    println!("Hello, world!");
}
