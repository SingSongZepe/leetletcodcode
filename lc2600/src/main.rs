mod test;

struct Solution;

impl Solution {
	pub fn k_items_with_maximum_sum(num_ones: i32, num_zeros: i32, num_neg_ones: i32, k: i32) -> i32 {
		if num_ones + num_zeros >= k {
			k.min(num_ones)
		} else {
			num_ones + num_ones + num_zeros - k
		}
	}
}

fn main() {
    println!("Hello, world!");
}
