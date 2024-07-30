mod test;

struct Solution;

impl Solution {
	pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
		let mut result: i64 = 0;
		let mut last_diff = std::i64::MAX; // the min value if count is odd, otherwise the max value
		let mut count = 0;  // the times that i ^ k
		for i in nums {
			let pov: i64 = i as i64 ^ k as i64;
			if pov > i as i64 {
				result += pov;
				count += 1;
				last_diff = last_diff.min(pov - i as i64);
			} else {
				result += i as i64;
				last_diff = last_diff.min(i as i64 - pov);
			}
		}
		if count % 2 == 1 {
			result - last_diff
		} else {
			result
		}
	}
}

struct Solution1;

impl Solution1 {
	pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
		let k = k as i64;
		let mut res= 0;
		let mut sac= std::i64::MAX;
		let mut count= 0;
		for &i in &nums{
			let i = i as i64;
			let fi= i ^ k;
			if fi > i{
				res += fi;
				count += 1;
				sac = std::cmp::min(sac, fi-i);
			}else{
				res += i;
				sac = std::cmp::min(sac, i-fi);
			}
		}
		if count % 2 == 1{
			res - sac
		}else{
			res
		}
	}
}

fn main() {
    println!("Hello, world!");
}
