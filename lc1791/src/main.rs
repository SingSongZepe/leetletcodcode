mod test;

struct Solution;

impl Solution {
	pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
		*edges[0].iter().find(|&&x| edges[1].contains(&x)).unwrap()
	}
}


fn main() {
    println!("Hello, world!");
}
