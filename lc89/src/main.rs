mod test;

struct Solution;

impl Solution {
	pub fn gray_code(n: i32) -> Vec<i32> {
		if n == 1 {
			return vec![0, 1];
		}
		let mut result = Self::gray_code(n-1);
		for i in (0..result.len()).rev() {
			result.push(result[i] + (1 << n-1));
		}
		result
	}
}

struct Solution1;

impl Solution1 {
	pub fn gray_code(n: i32) -> Vec<i32> {
		(0..(1<<n)).map(into_gray).collect()
	}
}
#[inline]
fn into_gray(n : i32) -> i32 {
	n ^ (n >> 1)
}

fn main() {
    println!("Hello, world!");
}
