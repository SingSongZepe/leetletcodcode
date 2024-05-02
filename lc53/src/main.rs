use log::log;

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut l,  mut r) = (0, nums.len()-1);
        
        loop {

        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let nums = vec![-2,1,-3,4,-1,2,1,-5,4];
        let result = Solution::max_sub_array(nums);
        println!("{}", result); // excepted 6
    }

    #[test]
    fn test2() {
        let nums = vec![1];
        let result = Solution::max_sub_array(nums);
        println!("{}", result); // 1
    }

    #[test]
    fn test3() {
        let nums = vec![5,4,-1,7,8];
        let result = Solution::max_sub_array(nums);
        println!("{}", result); // 23
    }
}

fn main() {
    println!("Hello, world!");
}
