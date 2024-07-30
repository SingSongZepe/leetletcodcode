mod test;

struct Solution;

impl Solution {
	// n^2 bad time complexity
	pub fn min_operations(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i64> {
		queries.iter().fold(vec![], |mut acc, val| {
			let mut t : i64 = 0;
			for &n in &nums {
				t += (n - val).abs() as i64;
			}
			acc.push(t);
			acc
		})
	}
}

struct Solution1;

impl Solution1 {
	pub fn min_operations(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i64> {
		nums.sort_unstable();  // O(nlogn)
		let (nl, ql) = (nums.len(), queries.len());
		let mut prefix_sums: Vec<i64> = Vec::with_capacity(nl);
		nums.iter().fold(0i64, |acc, &val| {
			prefix_sums.push(acc + val as i64);
			acc + val as i64
		}); // O(n)
		queries.iter().fold(Vec::<i64>::with_capacity(ql), |mut acc, &q| {
			if let Some(idx) = Self::find_index(&nums, q) {
				acc.push(((idx << 1) as i64 + 2 - nl as i64) * q as i64 - 2 * prefix_sums[idx] + prefix_sums[nl-1]);
				acc
			} else {
				acc.push(prefix_sums[nl-1] - nl as i64 * q as i64);
				acc
			}
		})
	}
	fn find_index(nums: &Vec<i32>, n: i32) -> Option<usize> {
		let mut result: Option<usize> = None;
		let mut low: i32 = 0;
		let mut high: i32 = nums.len() as i32 - 1;
		while low <= high {
			let mid = low + (high - low) / 2;
			if nums[mid as usize] < n {
				result = Some(mid as usize);
				low = mid + 1;
			} else {
				high = mid - 1;
			}
		}
		result
	}
}

struct Solution2;

impl Solution2 {
	pub fn min_operations(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i64> {
		nums.sort_unstable();  // O(nlogn)
		let (nl, ql) = (nums.len(), queries.len());
		let mut prefix_sums: Vec<i64> = Vec::with_capacity(nl);
		nums.iter().fold(0i64, |acc, &val| {
			prefix_sums.push(acc + val as i64);
			acc + val as i64
		}); // O(n)
		queries.iter().fold(Vec::<i64>::with_capacity(ql), |mut acc, &q| {
			if let Some(idx) = Self::find_index(&nums, q) {
				let val1 = (idx + 1) as i64 * q as i64 -  prefix_sums[idx];
				let val2 = prefix_sums[nl-1] - prefix_sums[idx] - (nl - idx - 1) as i64 * q as i64;
				acc.push(val1 + val2);
				acc
			} else {
				acc.push(prefix_sums[nl-1] - nl as i64 * q as i64);
				acc
			}
		})
	}
	fn find_index(nums: &Vec<i32>, n: i32) -> Option<usize> {
		let mut result: Option<usize> = None;
		let mut low: i32 = 0;
		let mut high: i32 = nums.len() as i32 - 1;
		while low <= high {
			let mid = low + (high - low) / 2;
			if nums[mid as usize] < n {
				result = Some(mid as usize);
				low = mid + 1;
			} else {
				high = mid - 1;
			}
		}
		result
	}
}

fn main() {
    println!("Hello, world!");
}
