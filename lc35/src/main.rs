use log::trace;

struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        Solution::bs(&nums, target, 0, nums.len() as usize - 1) as i32
    }

    pub fn bs(nums: &Vec<i32>, target: i32, left: usize, right: usize) -> usize {
        if left >= right {
            if target > nums[left] {
                return left + 1;
            }
            return left;
        }

        let mid = (left + right) / 2;
        if nums[mid] == target {
            mid
        } else if nums[mid] < target {
            Solution::bs(nums, target, mid + 1, right)
        } else {
            Solution::bs(nums, target, left, mid)
        }
    }

    pub fn search_insert1(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len());
        while l < r {
            let mid = l + ((r - l) >> 1);
            if nums[mid] < target {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        l as _
    }

    pub fn return_usize() -> usize {
        let i: i32 = 1;
        i as _
    }
    pub fn return_i64() -> i64 {
        let i: i32 = 1;
        i as _
    }
    pub fn return_i32() -> i32 {
        let i: usize = 1;
        i as _
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        let result = Solution::search_insert(nums, target);
        println!("{}", result); // excepted 2
    }

    #[test]
    fn test2() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;
        let result = Solution::search_insert(nums, target);
        println!("{}", result); // excepted 2
    }


    #[test]
    fn test3() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;
        let result = Solution::search_insert(nums, target);
        println!("{}", result); // excepted 2
    }

    #[test]
    fn test4() {
        let nums = vec![1, 3, 5, 6];
        let target = -1;
        let result = Solution::search_insert(nums, target);
        println!("{}", result); // excepted 2
    }
}

fn main() {
    println!("Hello, world!");
}
