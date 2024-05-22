mod test;

struct Solution;

impl Solution {
	pub fn partition(s: String) -> Vec<Vec<String>> {
		let s = s.as_bytes();
		let mut res = vec![];
		Self::recv(s, 0, &mut vec![], &mut res);
		res
	}
	fn recv(s: &[u8], from: usize, path: &mut Vec<String>, res: &mut Vec<Vec<String>>) {
		if from == s.len() {
			res.push(path.clone());
			return;
		}
		for to in from+1..=s.len() {
			if Self::is_palindrome(&s[from..to]) {
				path.push(String::from_utf8(s[from..to].to_vec()).unwrap());
				Self::recv(s, to, path, res);
				path.pop();
			}
		}
	}
	fn is_palindrome(s: &[u8]) -> bool {
		for i in 0..s.len()/2 {
			if s[i] != s[s.len()-i-1] {
				return false;
			}
		} true
	}
}

struct Solution1;

impl Solution1 {
	pub fn partition(s: String) -> Vec<Vec<String>> {
		// Time Complexity: O(n * 2^n)
		// Space Complexity: O(n^2)

		let mut result = vec![];
		let mut path = vec![];

		fn is_palindrome(s: &[u8]) -> bool {
			let len = s.len();
			for i in 0..len / 2 {
				if s[i] != s[len - 1 - i] {
					return false;
				}
			} true
		}

		fn backtrack(s: &[u8], start: usize, path: &mut Vec<String>, result: &mut Vec<Vec<String>>) {
			if start == s.len() {
				result.push(path.clone());
				return;
			}
			for end in (start + 1)..=s.len() {
				if is_palindrome(&s[start..end]) {
					path.push(String::from_utf8(s[start..end].to_vec()).unwrap());
					backtrack(s, end, path, result);
					path.pop();
				}
			}
		}

		backtrack(s.as_bytes(), 0, &mut path, &mut result);
		result
	}
}

fn main() {
    println!("Hello, world!");
}
