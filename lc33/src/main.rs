
struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len();

        let mid = (r + l) / 2;

        if nums[mid] >= nums[l] {
            if nums[l] <= target && target <= nums[mid] {
                r = mid;

            } else {

            }

        }


        0
    }

}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let nums = vec![4,5,6,7,0,1,2];
        let target = 0;
        let result = Solution::search(nums, target);
        println!("{}", result); //excepted 4
    }

    #[test]
    fn test2() {
        let nums = vec![4,5,6,7,0,1,2];
        let target = 3;
        let result = Solution::search(nums, target);
        println!("{}", result); //excepted -1
    }

    #[test]
    fn test3() {
        let nums = vec![1];
        let target = 0;
        let result = Solution::search(nums, target);
        println!("{}", result); //excepted -1
    }
}