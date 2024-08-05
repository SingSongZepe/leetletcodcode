
struct Solution;
struct Solution1;

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let (mut bal, mut cnt) = (0, 0);
		
		for c in s.chars() {
			match c {
				'L' => bal += 1,
				_ => bal -= 1,
			}
			cnt += if bal == 0 { 1 } else { 0 };
		}
		
		cnt
    }
}


impl Solution1 {
    pub fn balanced_string_split(s: String) -> i32 {
		s.chars().fold((0, 0), |(bal, cnt), c| match c {
			'L' => (bal + 1, cnt),
			_ => (bal - 1, cnt + if bal == 0 { 1 } else { 0 }),
		}).1
	}
}

fn main() {
    println!("Hello, world!");
}
