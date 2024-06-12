use std::mem::swap;

mod test;

struct Solution;

impl Solution {
	pub fn reverse_vowels(s: String) -> String {
		let is_vowel = |c: u8| -> bool {
			match c {
				b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U' => true,
				_ => false,
			}
		};
		let (mut l, mut r) = (0, s.len() - 1);
		let mut s = s.into_bytes();
		while l < r {
			while l < r && !is_vowel(s[l]) {
				l += 1;
			}
			while l < r && !is_vowel(s[r]) {
				r -= 1;
			}
			if l < r {
				s.swap(l, r);
				l += 1;
				r -= 1;
			}
		}
		String::from_utf8(s).unwrap()
	}
}

fn main() {
    println!("Hello, world!");
}
