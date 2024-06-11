mod test;

struct Solution;

impl Solution {
	pub fn convert_to_title(mut column_number: i32) -> String {
		let mut s = String::new();
		while column_number > 0 {
			column_number -= 1;
			s.push((((column_number) % 26) as u8 + 65) as _);
			column_number /= 26;
		}
		s.chars().rev().collect()
	}
}

struct Solution1;

impl Solution1 {
	pub fn convert_to_title(mut column_number: i32) -> String {
		if column_number == 0 {
			return String::new();
		}
		column_number -= 1;
		let ch = ((column_number % 26) as u8 + 65) as char;
		Self::convert_to_title(column_number / 26) + &ch.to_string()
	}
}

fn main() {
    println!("Hello, world!");
}
