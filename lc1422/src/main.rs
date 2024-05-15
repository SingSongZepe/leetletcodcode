mod test;

struct Solution;

impl Solution {
	pub fn max_score(s: String) -> i32 {
		let mut count_0: i32 = if s.chars().next() == Some('0') { 1 } else { 0 };
		for (idx, c) in s[..s.len()-1].chars().enumerate().skip(1) {
			if c == '1' {
				return count_0 + s[idx..].chars().filter(|&c| c == '1').count() as i32;
			}
			count_0 += 1;
		}
		count_0
	}
}

struct Solution1;

impl Solution1 {
	pub fn max_score(s: String) -> i32 {
		s[..s.len()-1].chars() // .into_iter()
			.fold((0, s.chars().filter(|&c| c == '1').count() as i32, i32::MIN),
			    |(x, y, result), c| {
				    match c {
				  	  '1' => (x, y-1, result.max(x+y-1)),
					  '0' => (x+1, y, result.max(x+y+1)),
					  _ => panic!("unknown char"),
				    }
		  	    }).2
	}
}

fn main() {
    println!("Hello, world!");
}
