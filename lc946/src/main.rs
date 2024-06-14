mod test;

struct Solution;

impl Solution {
	pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
		let mut stack = vec![pushed[0]];
		let (mut p1, mut p2) = (1, 0);
		let l = pushed.len();
		while !stack.is_empty() || p1 < l {
			if p2 < l && (!stack.is_empty() && popped[p2] == stack[stack.len()-1]) {
				stack.pop();
				p2 += 1;
			} else if p1 < l {
				stack.push(pushed[p1]);
				p1 += 1;
			} else {
				return false;
			}
		}
		true
	}
}

struct Solution1;

impl Solution1 {
	pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
		let mut stack = vec![];
		let mut pushed_iter = pushed.into_iter();
		for num in popped {
			while !stack.last().map_or(false, |&x| x == num) {
				if let Some(pnum) = pushed_iter.next() {
					stack.push(pnum);
				} else {
					return false;
				}
			}
			stack.pop();
		}
		true
	}
}

fn main() {
    println!("Hello, world!");
}
