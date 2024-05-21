mod test;

struct Solution;

impl Solution {
	pub fn beautiful_subsets(mut nums: Vec<i32>, k: i32) -> i32 {
		nums.sort_unstable(); // why we need to sort the nums?
		let mut result = 0;
		let mut visited = [false; 1000];
		Self::make(&nums, k, 0, &mut vec![], &mut visited, &mut result);
		result
	}
	pub fn make(nums: &Vec<i32>, k: i32, from: usize, path: &mut Vec<i32>, visited: &mut [bool], result: &mut i32) {
		if from == nums.len() {
			if !path.is_empty() {
				*result += 1;
			}
			return;
		}
		Self::make(nums, k, from+1, path, visited, result);
		if !(nums[from] + k < 1001 && visited[nums[from] as usize + k as usize - 1] || ( nums[from] > k && visited[nums[from] as usize - k as usize - 1])) {
			path.push(nums[from]);
			visited[nums[from] as usize - 1] = true;
			Self::make(nums, k, from+1, path, visited, result);
			path.pop();
			visited[nums[from] as usize - 1] = false;
		}
	}

	pub fn beautiful_subsets_unsorted(mut nums: Vec<i32>, k: i32) -> i32 {
		// nums.sort_unstable();
		let mut result = 0;
		let mut visited = [false; 1000];
		let mut real_result = vec![];
		Self::make_with_real_result(&nums, k, 0, &mut vec![], &mut visited, &mut result, &mut real_result);
		println!("{real_result:?}");
		result
	}
	pub fn make_with_real_result(nums: &Vec<i32>, k: i32, from: usize, path: &mut Vec<i32>, visited: &mut [bool], result: &mut i32, real_result: &mut Vec<Vec<i32>>) {
		if from == nums.len() {
			if !path.is_empty() {
				real_result.push(path.clone());
				*result += 1;
			}
			return;
		}
		Self::make_with_real_result(nums, k, from+1, path, visited, result, real_result);
		if !(nums[from] + k < 1001 && visited[nums[from] as usize + k as usize - 1] || ( nums[from] > k && visited[nums[from] as usize - k as usize - 1])) {
			path.push(nums[from]);
			visited[nums[from] as usize - 1] = true;
			Self::make_with_real_result(nums, k, from+1, path, visited, result, real_result);
			path.pop();
			visited[nums[from] as usize - 1] = false;
		}
	}
}

fn main() {
    println!("Hello, world!");
}
