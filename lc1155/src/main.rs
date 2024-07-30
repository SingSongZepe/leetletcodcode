mod test;

struct Solution;
use std::collections::HashMap;

impl Solution {
	pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
		let mut hm: HashMap<(i32, i32), i32> = HashMap::new();
		let result = Solution::make(n, k, target, &mut hm);
		println!("{}", result);
		result as i32
	}
	pub fn make(n: i32, k: i32, target: i32, hm: &mut HashMap<(i32, i32), i32>) -> i32 {
		if n == 1 {
			return 1;
		}
		if let Some(val) = hm.get(&(n, target)) {
			return *val;
		}

		let mut total: i32 = 0;
		for i in 1..=k.min(target-n+1) {
			total += Solution::make(n-1, k, target-i, hm);
		}

		hm.insert((n, target), total);
		total
	}
}

struct Solution1;

const MOD: i32 = 1000000007;

impl Solution1 {
	pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
		let mut dp: HashMap<(usize, usize), i32> = HashMap::new();

		let n = n as usize;
		let k = k as usize;
		let target = target as usize;

		for n in 2..=n {
			for target in 1..=target {
				if target > k * n {
					break;
				}
				for k in 1..=k.min(target - 1) {
					if n == 1 {
						dp.insert((n, target), 1);
					}
					let entry_ = *dp.get(&(n-1, target-k)).unwrap();
					let entry = dp.entry((n, target)).or_insert(0);
					*entry += entry_;
					*entry %= MOD;
				}
			}
		}

		*dp.get(&(n, target)).unwrap()
	}
}

struct Solution2;

impl Solution2 {
	pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
		if n * k < target || target < n {
			return 0;
		}
		if n * k == target || n == target {
			return 1;
		}

		let (target, n, k) = (target as usize, n as usize, k as usize);

		let mut dp = vec![0; target + 1];
		for i in 1 ..= k.min(target) {
			dp[i] = 1;
		}

		for v in 1 .. n {
			let mut tmp = vec![0; target + 1];
			for i in 1 ..= k {
				for p in v ..= target as usize {
					if dp[p] == 0 || p + i > target {
						break;
					}
					tmp[p + i] = (tmp[p + i] + dp[p]) % 1000000007;
				}
			}
			dp = tmp;
		}

		return dp[target];
	}
}

fn main() {
    println!("Hello, world!");
}
