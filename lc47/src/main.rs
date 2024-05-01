
struct Solution;

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results = vec![];
        Solution::permutations(&mut nums, 0, &mut results);
        results
    }

    fn permutations(nums: &mut Vec<i32>, start: usize, result: &mut Vec<Vec<i32>>) {
        if start == nums.len() {
            result.push(nums.clone());
        } else {
            let mut visited = vec![false; nums.len()];
            for i in start..nums.len() {
                if visited[i] || (i > start && nums[start..i].iter().any(|&x| x == nums[i])) {
                    continue;
                }
                visited[i] = true;
                nums.swap(start, i);
                Solution::permutations(nums, start + 1, result);
                nums.swap(start, i);
            }
        }
    }

    // fn permutations1(nums: &mut Vec<i32>, len: usize, start: usize, result: &mut Vec<Vec<i32>>) {
    //     if start == len {
    //         result.push(nums.clone());
    //     } else {
    //         for i
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let nums = vec![1, 1, 2];
        let result = Solution::permute_unique(nums);
        println!("{result:?}");
    }
}

fn main() {
    println!("Hello, world!");
}
