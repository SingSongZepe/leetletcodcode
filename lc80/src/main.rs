

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }

        let mut pr = 0;
        let mut rcount = 1;

        for i in 1..nums.len() {
            if nums[i] == nums[pr] {
                if rcount < 2 {
                    pr += 1;
                    rcount += 1;
                }
            } else {
                pr += 1;
                rcount = 1;
            }
            nums[pr] = nums[i];
        }

        (pr + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Solution,
    };

    #[test]
    fn test1() {
        let mut nums = vec![1,1,2];
        let result = Solution::remove_duplicates(&mut nums);
        println!("{}, {:?}", result, nums )
    }

    #[test]
    fn test2() {
        let mut nums = vec![0,0,1,1,1,1,2,3,3];
        let result = Solution::remove_duplicates(&mut nums);
        println!("{}, {:?}", result, nums )
    }

    #[test]
    fn test3() {
        let mut nums = vec![1,1,2];
        let result = Solution::remove_duplicates(&mut nums);
        println!("{}, {:?}", result, nums )
    }
}


fn main() {
    println!("Hello, world!");
}
