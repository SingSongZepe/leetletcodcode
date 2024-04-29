

struct Solution;

impl Solution {
    // You must implement an algorithm that runs in O(n) time and uses O(1) auxiliary space.
    // if you want to sort it, you at lest get an algo that runs in O(nlog(n))
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {

        1
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
}

fn main() {
    println!("Hello, world!");
}
