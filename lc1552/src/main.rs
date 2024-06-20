mod test;

struct Solution;

impl Solution {
	pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
		if m == 2 {
			return position.iter().max().unwrap() - position.iter().min().unwrap();
		}

		position.sort_unstable();
		let (mut l, mut r) = (0, position[position.len() - 1] - position[0]);

		while l < r {
			let mid = (l + r) / 2 + 1;
			if (1..position.len()).fold((1, position[0]), |(placed, prev_pos), i| {
				if position[i] - prev_pos >= mid {
					(placed + 1, position[i])
				} else {
					(placed, prev_pos)
				}
			}).0 >= m {
				l = mid;
			} else {
				r = mid - 1;
			}
		}
		l
	}
}

fn main() {
    println!("Hello, world!");
}
