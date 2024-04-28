


struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }
        let len = nums.len();
        let mut mi = Solution::bs(&nums, target, 0, (len - 1) as usize);
        if mi == -1 {
            return vec![-1, -1];
        }
        let mut left = mi;
        while left > 0 && nums[left as usize - 1] == target { left -= 1; };
        while mi < len as i32 - 1 && nums[mi as usize + 1] == target { mi += 1; };

        vec![left as i32, mi as i32]
    }

    pub fn bs(nums:& Vec<i32>, target: i32, left: usize, right: usize) -> i32 {
        if left >= right {
            return -1;
        }

        let mid = (left + right) / 2;
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] < target {
            Solution::bs(nums, target, mid + 1, right)
        } else {
            Solution::bs(nums, target, left, mid)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 8;
        let result = Solution::search_range(nums, target);
        println!("{result:?}"); // excepted [3, 4]
    }

    #[test]
    fn test2() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 6;
        let result = Solution::search_range(nums, target);
        println!("{result:?}"); // excepted [-1, -1]
    }

    #[test]
    fn test3() {
        let nums = vec![];
        let target = 0;
        let result = Solution::search_range(nums, target);
        println!("{result:?}"); // excepted [-1, -1]
    }

    #[test]
    fn test4() {
        let nums = vec![1];
        let target = 1;
        let result = Solution::search_range(nums, target);
        println!("{result:?}"); // excepted [0, 0]
    }

    #[test]
    fn test5() {
        let nums = vec![1, 2, 3, 4];
        let target = 4;
        let result = Solution::search_range(nums, target);
        println!("{result:?}"); // excepted [0, 0]
    }

    #[test]
    fn test6() {
        let nums = vec![2, 2];
        let target = 1;
        let result = Solution::search_range(nums, target);
        println!("{result:?}"); // excepted [0, 0]
    }
}



fn main() {
    println!("Hello, world!");
}
