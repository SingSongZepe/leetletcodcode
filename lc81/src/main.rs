
struct Solution;

impl Solution {
    // binary search O(logn)
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        // you need to jump the first element or the last element, because they are not sorted
        let nums = &nums[nums.iter().position(|n| n != nums.last().unwrap()).unwrap_or(nums.len() - 1)..].to_vec();
        // important to find the middle element, and then you can divide into two parts and binary search
        let pivot = nums.partition_point(|n| n > nums.last().unwrap());
        // better to pass (0..pivot)
        (pivot > 0 && Self::bs(&nums, target, 0, pivot-1)) || Self::bs(&nums, target, pivot, nums.len() - 1)
    }
    pub fn bs(nums: &Vec<i32>, target: i32, left: usize, right: usize) -> bool {
        if left >= right {
            if nums[left] == target {
                return true;
            }
            return false;
        }
        let mid = (left + right) / 2;
        if nums[mid] == target {
            return true;
        } else if nums[mid] > target {
            return Self::bs(nums, target, left, mid);
        } else {
            return Self::bs(nums, target, mid + 1, right);
        }
    }
}

struct Solution1;

impl Solution1 {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let nums1 = &nums[nums.iter().position(|n| n != nums.last().unwrap()).unwrap_or(nums.len() - 1)..];

        // important to find the middle element, and then you can divide into two parts and binary search
        let pivot = nums1.partition_point(|n| n > nums1.last().unwrap());
        nums1[..pivot].binary_search(&target).is_ok() || nums1[pivot..].binary_search(&target).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering::{Greater, Less};
    use crate::{
        Solution,
        Solution1,
    };

    #[test]
    fn test1() {
        let nums = vec![2, 5, 6, 0, 0, 1, 2];
        let target = 0;
        let result = Solution::search(nums, target);
        println!("{:?}", result);
    }

    #[test]
    fn test2() {
        let nums = vec![2, 5, 6, 0, 0, 1, 2];
        let target = 3;
        let result = Solution::search(nums, target);
        println!("{:?}", result);
    }

    #[test]
    fn test3() {
        let nums = vec![1];
        let target = 0;
        let result = Solution::search(nums, target);
        println!("{:?}", result);
    }

    #[test]
    fn test4() {
        let nums = vec![1];
        let target = 1;
        let result = Solution::search(nums, target);
        println!("{:?}", result);
    }

    #[test]
    fn test5() {
        let nums = vec![1,1,1,1,1,1,1,1,1,1,1,1,1,2,1,1,1,1,1];
        let target = 2;
        let result = Solution::search(nums, target);
        println!("{:?}", result);
    }

    #[test]
    fn test11() {
        let nums = vec![2, 5, 6, 0, 0, 1, 2];
        let target = 0;
        let result = Solution1::search(nums, target);
        println!("{:?}", result);
    }

    #[test]
    fn test12() {
        let nums = vec![2, 5, 6, 0, 0, 1, 2];
        let targer = 3;
        let result = Solution1::search(nums, targer);
        println!("{:?}", result);
    }

    #[test]
    fn test31() {
        let nums = vec![1];
        let target = 0;
        let result = Solution1::search(nums, target);
        println!("{:?}", result);
    }

    #[test]
    fn test41() {
        let nums = vec![1];
        let target = 0;
        let result = Solution1::search(nums, target);
        println!("{:?}", result);
    }

    #[test]
    fn test51() {
        let nums = vec![1,1,1,1,1,1,1,1,1,1,1,1,1,2,1,1,1,1,1];
        let target = 2;
        let result = Solution1::search(nums, target);
        println!("{:?}", result);
    }

    #[test]
    fn test_partition_point1() {
        let nums = vec![2, 5, 6, 0, 0, 1, 2];
        let result = nums.partition_point(|x| x > nums.last().unwrap());
        println!("{:?}", result);
    }

    #[test]
    fn test_partition_point2() {
        let nums = vec![2, 5, 6, 0, 0, 1, 2];
        let result = nums.partition_point(|x| *x > 5);
        println!("{:?}", result);
    }

    #[test]
    fn test_partition_point_implement() {
        let nums = vec![2, 5, 6, 0, 0, 1, 2];
        let pred = |x: &i32| *x > 2;
        let result = nums.binary_search_by(
            |x| {
                if *x > 2 {
                    Less
                } else {
                    Greater
                }
            }
        )
            .unwrap_or_else(|i| i);
        println!("{}", result);
    }
}

fn main() {
    println!("Hello, world!");
}
