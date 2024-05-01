

struct Solution;

use std::collections::HashMap;
impl Solution {
    // You must implement an algorithm that runs in O(n) time and uses O(1) auxiliary space.
    // if you want to sort it, you at lest get an algo that runs in O(nlog(n))


    // but just beats 6.8% at speed
    // just beats 5.9% at memory
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut hash = HashMap::<i32, bool>::new();
        for i in nums {
            hash.insert(i, true);
        }
        let mut i = 1;
        while let Some(_) = hash.get(&i) { i += 1 };
        i
    }
}

impl Solution {
    pub fn first_missing_positive1(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();

        // Making non-positive values positive and large than `len`
        for n in &mut nums {
            if *n <= 0 {
                *n = i32::MAX;
            }
        }

        // Using sign as Set
        for i in 0..len {
            let n = (nums[i].abs() - 1) as usize;
            if n >= len {
                continue;
            }
            nums[n] = -nums[n].abs(); // there nums[n] will be used in the after array
        }
        println!("{:?}", nums);

        // Find positive number with smallest index
        (nums.into_iter().position(|x| x > 0).unwrap_or(len) + 1) as i32
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 0];
        let result = Solution::first_missing_positive(nums);
        println!("{result}"); // expected 3
    }

    #[test]
    fn test2() {
        let nums = vec![3,4,-1,1];
        let result = Solution::first_missing_positive(nums);
        println!("{result}"); // expected 2
    }

    #[test]
    fn test3() {
        let nums = vec![7,8,9,11,12];
        let result = Solution::first_missing_positive(nums);
        println!("{result}"); // expected 1
    }

    #[test]
    fn test4() {
        let nums = vec![1, 2, 0];
        let result = Solution::first_missing_positive1(nums);
        println!("{result}"); // expected 3
    }

    #[test]
    fn test5() {
        let nums = vec![3,4,-1,1];
        let result = Solution::first_missing_positive1(nums);
        println!("{result}"); // expected 2
    }

    #[test]
    fn test6() {
        let nums = vec![7,8,9,11,12];
        let result = Solution::first_missing_positive1(nums);
        println!("{result}"); // expected 1
    }

    #[test]
    fn test7() {
        let nums = vec![2, 2];
        let result = Solution::first_missing_positive1(nums);
        println!("{result}"); // expected 1
    }

    #[test]
    fn test_position() {
        let pos = vec![1, 4, -4, 2, 5, 9];
        let i = pos.iter().position(|x| *x > 4).unwrap_or(pos.len());
        println!("{}", i);
    }
}

fn main() {
    println!("Hello, world!");
}
