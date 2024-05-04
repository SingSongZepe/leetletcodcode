
struct Solution;

impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let len = nums.len();
        Solution::permutations(&mut nums, len, 0, &mut result);
        result
    }

    pub fn permutations(nums: &mut Vec<i32>, len: usize, start: usize, result: &mut Vec<Vec<i32>>) {
        if start == len {
            result.push(nums.clone());
        } else {
            for i in start..nums.len() {
                nums.swap(start, i);
                Solution::permutations(nums, len, start + 1, result);
                nums.swap(start, i);
            }
        }
    }

    pub fn permutations1(nums: &mut Vec<i32>, len: usize, start: usize, result: &mut Vec<Vec<i32>>) {
        if start == len {
            result.push(nums.clone());
        } else {
            for i in start..nums.len() {
                nums.swap(start, i);
                Solution::permutations1(nums, len, start + 1, result);
                nums.swap(start, i);
            }
        }
    }
}


impl Solution {
    pub fn permute1(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut path = Vec::new();
        let mut used = vec![false; nums.len()];
        Solution::dfs(&nums, &mut path, &mut used, &mut result);
        result
    }

    fn dfs(nums: &Vec<i32>, path: &mut Vec<i32>, used: &mut Vec<bool>, result: &mut Vec<Vec<i32>>) {
        if path.len() == nums.len() {
            result.push(path.clone());
            return;
        }

        for i in 0..nums.len() {
            if !used[i] {
                path.push(nums[i]);
                used[i] = true;
                Solution::dfs(nums, path, used, result);
                path.pop();
                used[i] = false;
            }
        }
    }
}

use itertools::Itertools;
#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use crate::Solution;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 3];
        let result = Solution::permute(nums);
        println!("{result:?}");
    }

    #[test]
    fn test2() {
        let nums = vec![0, 1];
        let result = Solution::permute(nums);
        println!("{result:?}");
    }

    #[test]
    fn test3() {
        let nums = vec![1];
        let result = Solution::permute(nums);
        println!("{result:?}");
    }

    #[test]
    fn test11() {
        let nums = vec![1, 2, 3];
        let result = Solution::permute1(nums);
        println!("{result:?}");
    }

    #[test]
    fn test21() {
        let nums = vec![0, 1];
        let result = Solution::permute1(nums);
        println!("{result:?}");
    }

    #[test]
    fn test31() {
        let nums = vec![1];
        let result = Solution::permute1(nums);
        println!("{result:?}");
    }

    #[test]
    fn test_permutation() {
        let mut nums = vec![1, 2, 3];
        let mut results: Vec<Vec<i32>> = vec![];
        let len = nums.len();
        // let mut permutations: Vec<Vec<i32>> = nums.iter().permutations(1).collect();
        Solution::permutations(&mut nums, len, 0, &mut results);
        println!("{results:?}")
    }
}

fn main() {
    println!("Hello, world!");
}
