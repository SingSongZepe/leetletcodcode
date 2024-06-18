mod test;

struct Solution;

impl Solution {
	pub fn decode(mut encoded: Vec<i32>, first: i32) -> Vec<i32> {
		encoded.into_iter().fold(vec![first], |mut acc, i| {
			acc.push(acc[acc.len() - 1] ^ i);
			acc
		})
	}
}

struct Solution1;

impl Solution1 {
	pub fn decode(mut encoded: Vec<i32>, first: i32) -> Vec<i32> {
		for i in 0..encoded.len() {
			encoded[i] ^= if i == 0 { first } else { encoded[i - 1] };
		}
		let mut result = vec![first];
		result.extend(encoded);
		result
	}
}

fn main() {
    println!("Hello, world!");
}
