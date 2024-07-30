mod test;

struct Solution;

impl Solution {
	pub fn min_cut(s: String) -> i32 {
		let (mut curr_depth, mut min_depth) = (0, i32::MAX);
		Self::recv(&s.as_bytes(), 0, &mut curr_depth, &mut min_depth);
		min_depth - 1
	}
	fn recv(s: &[u8], from: usize, curr_depth: &mut i32, min_depth: &mut i32) {
		if s.len() == from {
			*min_depth = (*curr_depth).min(*min_depth);
			return;
		}
		for to in (from+1..=s.len()).rev() {
			if Self::is_palindrome(&s[from..to]) {
				*curr_depth += 1;
				Self::recv(s, to, curr_depth, min_depth);
				*curr_depth -= 1;
			}
		}
	}
	fn is_palindrome(s: &[u8]) -> bool {
		let len = s.len();
		for i in 0..len/2 {
			if s[i] != s[len-i-1] {
				return false;
			}
		} true
	}
}

struct Solution1;

impl Solution1 {
	pub fn min_cut(s: String) -> i32 {
		let s = s.as_bytes();
		let mut dp = vec![i32::MAX; s.len() + 1];
		dp[0] = -1;

		for i in 0..s.len() {
			for j in (0..=i).rev() {
				if Self::is_palindrome(&s[j..=i]) {
					dp[i + 1] = dp[i + 1].min(dp[j] + 1);
				}
			}
		}
		dp[s.len()]
	}
	fn is_palindrome(s: &[u8]) -> bool {
		let len = s.len();
		for i in 0..len/2 {
			if s[i] != s[len-i-1] {
				return false;
			}
		} true
	}
}

struct Solution2;

impl Solution2 {
	pub fn min_cut(s: String) -> i32 {
		let s = s.as_bytes();
		let n = s.len();
		let mut dp = vec![i32::MAX; n];
		let mut is_palindrome = vec![vec![false; n]; n];

		for i in (0..n).rev() {
			for j in i..n {
				if s[i] == s[j] && (j - i <= 1 || is_palindrome[i + 1][j - 1]) {
					is_palindrome[i][j] = true;
				}
			}
		}

		for i in 0..n {
			if is_palindrome[0][i] {
				dp[i] = 0;
			} else {
				for j in 0..i {
					if is_palindrome[j + 1][i] {
						dp[i] = dp[i].min(dp[j] + 1);
					}
				}
			}
		}

		dp[n - 1]
	}
}

fn main() {
    println!("Hello, world!");
}
