use std::fmt::format;

struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 1 {
            return 0;
        }
        let mut from = 0;
        let mut time = 0;
        while from < len {
            let mut im = from+1;
            if from + nums[from] as usize >= len - 1 {
                time += 1;
                return time;
            }
            for i in (from+1..=from+nums[from] as usize).rev() {
                if nums[i] != 0 && i + nums[i] as usize >= im + nums[im] as usize {
                    im = i;
                }
            }
            from = im;
            time += 1;
        }
        time
    }
}

impl Solution {
    pub fn jump1(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 1 {
            return 0;
        }

        let mut from = 0;
        let mut time = 0;

        while from < len {
            let im = (from + 1..=from + nums[from] as usize)
                .rev()
                .filter(|&i| i < len && i + nums[i] as usize >= from + nums[from] as usize)
                .max_by_key(|&i| i + nums[i] as usize)
                .unwrap_or(0);

            if im + nums[im] as usize >= len - 1 {
                time += 1;
                return time;
            }

            from = im;
            time += 1;
        }

        time
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let nums = vec![2, 3, 1, 1, 4];
        let result = Solution::jump(nums);
        println!("{result}");
    }

    #[test]
    fn test2() {
        let nums = vec![2, 3, 0, 1, 4];
        let result = Solution::jump(nums);
        println!("{result}");
    }

    #[test]
    fn test3() {
        let nums = vec![5000, 1];
        let result = Solution::jump(nums);
        println!("{result}");
    }

    #[test]
    fn test4() {
        let nums = vec![1, 1, 1, 1, 1, 1, 1];
        let result = Solution::jump(nums);
        println!("{result}");
    }

    #[test]
    fn test5() {
        let nums = vec![1, 2, 3];
        let result = Solution::jump(nums);
        println!("{result}");
    }

    #[test]
    fn test6() {
        let nums = vec![3, 2, 1];
        let result = Solution::jump(nums);
        println!("{result}");
    }

    #[test]
    fn test7() {
        let nums = vec![0];
        let result = Solution::jump(nums);
        println!("{result}");
    }

    #[test]
    fn test11() {
        let nums = vec![2, 3, 1, 1, 4];
        let result = Solution::jump1(nums);
        println!("{result}");
    }

    #[test]
    fn test21() {
        let nums = vec![2, 3, 0, 1, 4];
        let result = Solution::jump1(nums);
        println!("{result}");
    }


    #[test]
    fn test31() {
        let nums = vec![5000, 1];
        let result = Solution::jump1(nums);
        println!("{result}");
    }

    #[test]
    fn test41() {
        let nums = vec![1, 1, 1, 1, 1, 1, 1];
        let result = Solution::jump1(nums);
        println!("{result}");
    }

    #[test]
    fn test51() {
        let nums = vec![1, 2, 3];
        let result = Solution::jump1(nums);
        println!("{result}");
    }

    #[test]
    fn test61() {
        let nums = vec![3, 2, 1];
        let result = Solution::jump1(nums);
        println!("{result}");
    }

    #[test]
    fn test71() {
        let nums = vec![0];
        let result = Solution::jump1(nums);
        println!("{result}");
    }
}

fn main() {
    println!("Hello, world!");
}
