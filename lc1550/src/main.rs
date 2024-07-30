mod test;

struct Solution;

impl Solution {
	pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
		let mut count = 0;
		for &num in &arr {
			if num % 2 == 1 {
				count += 1;
				if count == 3 {
					return true;
				}
			} else {
				count = 0;
			}

		}
		false
	}
}

struct Solution1;

impl Solution1 {
	pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
		arr.into_iter().fold(0, |count, num| {
			if count != 3 {
				if num % 2 == 1 {
					count + 1
				} else {
					0
				}
			} else {
				count
			}
		}) == 3
	}
}


fn main() {
    println!("Hello, world!");
}
