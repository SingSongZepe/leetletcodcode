mod test;

struct Solution;

impl Solution {
	pub fn is_anagram(s: String, t: String) -> bool {
		macro_rules! char2usize {
			($c: tt) => {
				$c as usize - 'a' as usize
			}
		}
		if s.len() != t.len() {
			return false;
		}
		let mut counts = [0; 26];
		s.chars().zip(t.chars()).for_each(|(c1, c2)| {
			counts[char2usize!(c1)] += 1;
			counts[char2usize!(c2)] -= 1;
		});
		counts.iter().all(|&count| count == 0)
	}
}

struct Solution1;

impl Solution1 {
	pub fn is_anagram(s: String, t: String) -> bool {
		macro_rules! char2usize {
			($c: tt) => {
				$c as usize - 'a' as usize
			}
		}
		s.len() == t.len() && s.chars().zip(t.chars()).fold([0; 26], |mut counts, (c1, c2)| { counts[char2usize!(c1)] += 1; counts[char2usize!(c2)] -= 1; counts }).iter().all(|&count| count == 0)
	}
}

fn main() {
    println!("Hello, world!");
}
