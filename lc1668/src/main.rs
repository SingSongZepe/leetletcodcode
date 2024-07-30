mod test;

struct Solution;

impl Solution {
	pub fn max_repeating(word: String, sequence: String) -> i32 {

		let sequence = sequence.as_bytes();
		let word = word.as_bytes();
		let (sl, wl) = (sequence.len(), word.len());
		let mut m = 0i32;

		let mut idx = 0;
		while idx < wl {
			let mut p = idx;
			while p < wl && word[p] == sequence[(p-idx)%sl] {
				p += 1;
			}
			m = m.max((p - idx) as i32 / sl as i32);
			idx = if p != idx {
				p - sl + 1
			} else {
				idx + 1
			}
		}
		m
	}
}

struct Solution2;

impl Solution2 {
	pub fn max_repeating(sequence: String, word: String) -> i32 {
		sequence
			.chars()
			.skip(word.len() - 1)
			.enumerate()
			.map(|(i, _)| {
				Self::count(&sequence[i..], &word)
			})
			.max()
			.unwrap_or_default()
	}
	fn count(sequence: &str, word: &str) -> i32 {
		sequence
			.strip_prefix(word)
			.map(|rest| Self::count(rest, word) + 1)
			.unwrap_or_default()
	}
}


struct Solution1;

impl Solution1 {
	pub fn max_repeating(sequence: String, word: String) -> i32 {
		sequence
			.chars()
			.skip(word.len() - 1)
			.enumerate()
			.map(|(i, _)| Self::count_repeating(&sequence[i..], &word))
			.max()
			.unwrap_or_default()
	}
	fn count_repeating(sequence: &str, word: &str) -> i32 {
		sequence
			.strip_prefix(word)
			.map(|rest| Self::count_repeating(rest, word) + 1)
			.unwrap_or_default()
	}
}

fn main() {
    println!("Hello, world!");
}
