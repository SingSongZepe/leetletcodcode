mod test;

struct Solution;

impl Solution {
	pub fn min_operations(s: String) -> i32 {
		let mut count1 = 0;
		let mut count2 = 0;
		for (idx, c) in s.chars().enumerate() {
			if idx % 2 == 0 {
				count1 += (c == '1') as i32;  // make 0101
				count2 += (c == '0') as i32;  // make 1010
			} else {
				count2 += (c == '1') as i32;
				count1 += (c == '0') as i32;
			}
		}
		count1.min(count2)
	}
}

fn main() {
    println!("Hello, world!");
}
