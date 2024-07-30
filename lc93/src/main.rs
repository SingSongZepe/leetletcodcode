use std::fmt::format;

mod test;

struct Solution;

impl Solution {
	pub fn restore_ip_addresses(s: String) -> Vec<String> {
		if s.len() > 12 || s.len() < 4 {
			return vec![];
		}
		let mut result = vec![];
		Self::make(s.as_str(), 0, 1, &mut vec![], &mut result, s.len());
		result
	}
	fn make(s: &str, from: usize, idx: usize, path: &mut Vec<String>, res: &mut Vec<String>, slen: usize) {
		if idx == 5 {
			if from == slen {
				res.push(path.join("."));
				return;
			} else {
				return;
			}
		}
		for i in (from+1).max((slen as i32-12+3*idx as i32).max(0) as usize)..=(from+3).min(slen-4+idx) { // cut
			let (b, sub) = Self::is_valid_ip_segment(&s[from..i]);
			if b {
				path.push(sub);
				Self::make(s, i, idx+1, path, res, slen);
				path.pop();
			}
		}
	}
	fn is_valid_ip_segment(segment: &str) -> (bool, String) {
		if segment.len() > 3 {
			return (false, segment.to_string());
		}
		if segment.starts_with('0') && segment.len() > 1 {
			return (false, segment.to_string());
		}
		if let Ok(num) = segment.parse::<i32>() {
			if num >= 0 && num <= 255 {
				return (true, num.to_string());
			}
		}
		(false, segment.to_string())
	}
}

fn main() {
    println!("Hello, world!");
}
