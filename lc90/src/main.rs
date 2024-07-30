mod test;

struct Solution;

impl Solution {
	pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
		nums.sort();
		let mut result = vec![];
		Solution::make_subsets(&nums, 0, &mut vec![], &mut result);
		result
	}
	pub fn make_subsets(nums: &Vec<i32>, start: usize, selected: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
		result.push(selected.clone());
		for i in start..nums.len() { // !
			if i != start && nums[i] == nums[i-1] {
				continue;
			}
			selected.push(nums[i]);
			Solution::make_subsets(nums, i+1, selected, result);
			selected.pop();
			// break;
		}
	}
}

struct Solution1;

impl Solution1 {
	fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
		let mut nums = nums;
		let mut res = Vec::new();
		let mut curr = Vec::new();
		nums.sort();  // 首先对nums排序
		Self::generate_subsets_with_dup(&nums, 0, &mut curr, &mut res); // 调用辅助函数生成子集
		res
	}

	fn generate_subsets_with_dup(nums: &Vec<i32>, start: usize, curr: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
		res.push(curr.clone());  // 将当前子集加入结果集

		let mut i = start;
		while i < nums.len() {
			if i == start || nums[i] != nums[i - 1] {  // 跳过重复元素
				curr.push(nums[i]);
				Self::generate_subsets_with_dup(nums, i + 1, curr, res);  // 递归生成子集
				curr.pop();
			}
			i += 1;
		}
	}
}

fn main() {
    println!("Hello, world!");
}
