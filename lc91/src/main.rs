mod test;

struct Solution;

impl Solution {
	pub fn num_decodings(s: String) -> i32 {
		// let mut memo =
		Self::figure(&s, 0, s.len()-1)
	}
	pub fn figure(s: &str, from: usize, to: usize) -> i32 {
		if to > from { // split
			return (from + 1..to).fold(0, |acc, i| acc + Self::figure(s, from, i-1) + Self::figure(s, i, to));
		}
		match to == from {
			true => {
				match &s[from..from+1].as_bytes()[0] {
					49..=57 => 1,
					_ => 0,
				}
			},
			_ => {
				match &s[from..from+1].as_bytes()[0] { // to = from + 1
					49 => 1,
					50 => { // '2'
						if (&s[from..from+1].as_bytes())[1] < 55 {
							1
						} else {
							0
						}
					}
					_ => 0,
				}
			}
		}
	}
}


struct Solution1;

impl Solution1 {
	pub fn num_decodings(s: String) -> i32 {
		let mut prev = 1;
		let mut prev_2 = 0;
		let mut prev_ch = 'X';
		let mut cur = 1;
		for ch in s.chars().rev() {
			if ch == '1' || (ch == '2' && prev_ch as u8 <= '6' as u8) {
				cur = prev_2 + prev;
			}
			else if ch == '0' {
				cur = 0;
			}
			prev_2 = prev;
			prev = cur;
			prev_ch = ch;
		}
		cur as _
	}
}

fn main() {
    println!("Hello, world!");
}
