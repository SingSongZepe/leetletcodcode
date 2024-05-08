
struct Solution;

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut si = score.iter().enumerate().collect::<Vec<_>>();
        si.sort_by(|a, b| b.1.cmp(a.1));
        let mut result = vec![String::new(); score.len()];
        for (idx, s) in si.iter().enumerate() {
            result[s.0] = match idx {
                0 => "Gold Medal".to_string(),
                1 => "Silver Medal".to_string(),
                2 => "Bronze Medal".to_string(),
                _ => (idx + 1).to_string(),
            }
        }
        result
    }
}

use std::collections::BinaryHeap;

struct Solution1;

impl Solution1 {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut heap = BinaryHeap::from_iter(score.iter().cloned().enumerate().map(|(a, b)| (b, a)));
        // let ranks = [String::from("Gold Medal"), String::from("Silver Medal"), String::from("Bronze Medal")];
        let mut res = vec![String::new(); score.len()];
        (0..score.len()).for_each(|i| res[heap.pop().unwrap().1] = match i {
            0 => "Gold Medal".to_string(),
            1 => "Silver Medal".to_string(),
            2 => "Bronze Medal".to_string(),
            _ => (i + 1).to_string(),
        });
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Solution,
        Solution1,
    };

    #[test]
    fn test1() {
        let score = vec![5, 4, 3, 2, 1];
        let result = Solution::find_relative_ranks(score);
        println!("{:?}", result); // expeced output: ["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]
    }

    #[test]
    fn test2() {
        let score = vec![10, 3, 8, 9, 4];
        let result = Solution::find_relative_ranks(score);
        println!("{:?}", result); // expected output: ["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"]
    }

    #[test]
    fn test11() {
        let score = vec![5, 4, 3, 2, 1];
        let result = Solution1::find_relative_ranks(score);
        println!("{:?}", result); // expeced output: ["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]
    }

    #[test]
    fn test21() {
        let score = vec![10, 3, 8, 9, 4];
        let result = Solution1::find_relative_ranks(score);
        println!("{:?}", result); // expected output: ["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"]
    }
}

fn main() {
    println!("Hello, world!");
}
