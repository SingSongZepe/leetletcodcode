

struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut selected = vec![];
        Solution::make_subsets(&nums, 0, &mut selected, &mut result);
        result
    }
    pub fn make_subsets(nums: &Vec<i32>, start: usize, selected: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if start == nums.len() {
            result.push(selected.clone());
            return;
        }
        // not append
        Solution::make_subsets(nums, start+1, selected, result);

        // append
        selected.push(nums[start]);
        Solution::make_subsets(nums, start+1, selected, result);
        selected.pop();
    }
}

struct Solution1;

impl Solution1 {
    // bad method
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut selected = vec![];
        let mut visited = vec![false; nums.len()];
        Solution1::make_subsets(&nums, 0, &mut visited, &mut selected, &mut result);
        result
    }
    pub fn make_subsets(nums: &Vec<i32>, depth: usize, visited: &mut Vec<bool>, selected: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if depth == nums.len() {
            result.push(selected.clone());
            return;
        }
        // not append
        Solution1::make_subsets(nums, depth+1, visited, selected, result);

        // not append
        for i in depth..nums.len() {
            if !visited[i] {
                visited[i] = true;
                selected.push(nums[i]);
                Solution1::make_subsets(nums, depth+1, visited, &mut selected.clone(), result);
                visited[i] = false;
                selected.pop();
            }
        }
    }
}

struct Solution2;


impl Solution2 {
    // good method
    pub fn subsets(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 0{
            return vec![vec![]];
        }
        let last = nums.pop().unwrap();
        let remain_subsets = Solution2::subsets(nums);
        let mut res = remain_subsets.clone();
        for mut v in remain_subsets{
            v.push(last);
            res.push(v);
        }
        res
    }
}


#[cfg(test)]
mod tests {
    use crate::{
        Solution,
        Solution1,
        Solution2,
    };

    #[test]
    fn test1() {
        let nums = vec![1, 2, 3];
        let result = Solution::subsets(nums);
        println!("{:?}", result);
    }

    #[test]
    fn test2() {
        let nums = vec![0];
        let result = Solution::subsets(nums);
        println!("{:?}", result);
    }


    #[test]
    fn test11() {
        let nums = vec![1, 2, 3];
        let result = Solution1::subsets(nums);
        println!("{:?}", result);
    }

    #[test]
    fn test21() {
        let nums = vec![0];
        let result = Solution1::subsets(nums);
        println!("{:?}", result);
    }

    #[test]
    fn test12() {
        let nums = vec![1, 2, 3];
        let result = Solution2::subsets(nums);
        println!("{:?}", result);
    }

    #[test]
    fn test22() {
        let nums = vec![0];
        let result = Solution2::subsets(nums);
        println!("{:?}", result);
    }
}

fn main() {
    println!("Hello, world!");
}
