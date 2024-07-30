mod test;

struct Solution;

impl Solution {
	pub fn even_odd_bit(mut n: i32) -> Vec<i32> {
		let mut res = vec![0, 0];
		while n > 0 {
			if n & 1 == 1 {
				res[0] += 1;
			}
			if n & 2 == 2 {
				res[1] += 1;
			}
			n >>= 2;
		}
		res
	}
}

fn main() {
    println!("Hello, world!");
}
