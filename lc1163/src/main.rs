use std::process::id;

mod test;

struct Solution;

impl Solution {
	pub fn last_substring(s: String) -> String {
		let mut mchar: u8 = 0;
		let mut poss = vec![];
		for (idx, &ch) in s.as_bytes().iter().enumerate() {
			if ch > mchar {
				mchar = ch;
				poss.clear();
			}
			if ch == mchar {
				poss.push(idx);
			}
		}

		let mut competitors = vec![true; poss.len()];
		let mut count_comp = poss.len();
		for sublen in 1..=s.len() {
			if count_comp == 1 {
				break;
			}
			let mut mch = 0 as char;
			let mut midx = vec![];
			for (idx, &pos) in poss.iter().enumerate() {
				if !competitors[idx] {
					continue;
				}
				let p = pos + sublen;
				if p >= s.len() {
					competitors[idx] = false;
					count_comp -= 1;
					continue;
				}
				if s.chars().nth(p).unwrap() > mch {
					mch = s.chars().nth(p).unwrap();
					for idx in &midx {
						competitors[*idx] = false;
						count_comp -= 1;
					}
					midx.clear();
					midx.push(idx);
				} else if s.chars().nth(p).unwrap() == mch {
					midx.push(idx);
				} else {
					competitors[idx] = false;
					count_comp -= 1;
				}
			}
			midx.clear();
		}

		s[poss[competitors.iter().position(|&x| x).unwrap()]..].to_string()
	}
}

struct Solution1;

impl Solution1 {
	pub fn last_substring(s: String) -> String {
		let s = s.as_bytes();
		let mut head = 0;
		let mut tail = 1;

		while tail < s.len() {
			if s[tail] > s[head] {
				head = tail;
			} else if s[tail] == s[head] {
				if s[head..] < s[tail..] { // !!!!
					head = tail;
				}
			}
			tail += 1;
			while tail < s.len() && s[tail] < s[head] {
				tail += 1;
			}
		}
		String::from_utf8_lossy(&s[head..]).to_string()
	}
}

struct Solution2;

impl Solution2 {
	pub fn last_substring(s: String) -> String {
		let s = s.as_bytes();
		let mut res = &s[..];

		for (i, val) in s.iter().enumerate() {
			let max_val = if val > &s.iter().next().unwrap()
				&& s[i..] > s[i + 1..] {
				&s[i..] } else { &s[i + 1..] };
			res = res.max(max_val);
		}

		unsafe { String::from_utf8_unchecked(res.to_vec()) }
	}
}

fn main() {
    println!("Hello, world!");
}
