mod test;

struct Solution;

impl Solution {
	pub fn interpret(command: String) -> String {
		let mut result = String::new();
		// let mut i = 0;
		for (idx, c) in command.chars().enumerate() {
			if c == 'G' {
				result.push('G');
			} else if c == '(' {
				if &command[idx..=idx+1] == "()" {
					result.push('o');
				} else {
					result.push_str("al");
				}
			}
		}
		result
	}
}


struct Solution1;

impl Solution1 {
	pub fn interpret(command: String) -> String {
		command.replace("()", "o").replace("(al)", "al")
	}
}


struct Solution2;

impl Solution2 {
	pub fn interpret(command: String) -> String {
		command.chars().enumerate()
			.fold(String::new(), |mut result, (idx, c)| {
				match c {
					'G' => result.push('G'),
					'(' => {
						if &command[idx..=idx+1] == "()" {
							result.push('o')
						} else {
							result.push_str("al")
						}
					},
					_ => (),
				}
				result
			})
	}
}

fn main() {
    println!("Hello, world!");
}
