use std::{collections::HashMap, vec};

struct  Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_with_index: Vec<(usize, i32)> = nums.iter().enumerate().map(|(i, &val)| (i, val)).collect();
        nums_with_index.sort_by_key(|(_, val)| *val);
    
        let mut left = 0;
        let mut right = nums_with_index.len() - 1;
    
        while left < right {
            let sum = nums_with_index[left].1 + nums_with_index[right].1;
            if sum == target {
                return vec![nums_with_index[left].0 as i32, nums_with_index[right].0 as i32];
            } else if sum > target {
                right -= 1;
            } else {
                left += 1;
            }
        }
    
        vec![]
    }
    pub fn two_sum1(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashmap:HashMap<usize, i32> = HashMap::new();
        
        for (i, &val) in nums.iter().enumerate() {
            let complement :usize = (target - val) as usize;
            if let Some(&j) = hashmap.get(&complement) {
                return vec![j as i32, i as i32];
            }
            hashmap.insert(i, val);
        }

        vec![]
    }
    // pub fn hw() {
        // println!("hw");
    // }
}



fn main() {
    let v = vec![2,7,11,15];
    let t = 9; 
    // let v1 = Solution::two_sum(v, t);
    // println!("{v1:?}");
    
    let v2 = Solution::two_sum1(v, t);
    println!("{v2:?}");
    // Solution::hw();
}
