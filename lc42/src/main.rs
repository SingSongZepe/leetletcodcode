
struct Solution;


use std::cmp::min;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let len = height.len();
        let mut left = 0;
        let mut right = left;
        let mut total = 0;

        while right < len - 1 && height[right + 1] >= height[right] {
            right += 1;
        }
        left = right;
        right += 2;
        while right < len {
            let mut im = right;
            let mut setten = false;
            while right < len {
                if height[right] >= height[left] {
                    setten = true;
                    break;
                } else if height[right] >= height[im] {
                    im = right;
                }
                right += 1;
            }
            if !setten {
                right = im;
            }

            while left < right - 1 && height[left + 1] >= height[right] {
                left += 1;
            }

            total += min(height[right], height[left]) * (right as i32 - left as i32 - 1) - Solution::sum(&height, left, right);
            left = right;
            right += 2;
        }
        total
    }

    pub fn sum(height: &Vec<i32>, from: usize, to: usize) -> i32 {
        height[from+1..to].iter().fold(0, |x, i| x + *i)
    }
}

impl Solution {
    pub fn trap1(height: Vec<i32>) -> i32 {
        let (mut l, mut r, mut maxl, mut maxr, mut ans) = (0_usize, height.len()-1, 0, 0, 0);
        while l <= r {
            match height[l] <= height[r] {
                true if height[l] >= maxl =>  {
                    maxl = height[l];
                    l+=1;
                }
                true =>                {
                    ans += maxl - height[l];
                    l+=1;
                }
                false if height[r] >= maxr => {
                    maxr = height[r];
                    r-=1;
                }
                false =>               {
                    ans += maxr - height[r];
                    r-=1;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let height = vec![0,1,0,2,1,0,1,3,2,1,2,1];
        let result = Solution::trap(height);
        println!("{}", result);
    }

    #[test]
    fn test2() {
        let height = vec![4,2,0,3,2,5];
        let result = Solution::trap(height);
        println!("{}", result);
    }

    #[test]
    fn test3() {
        let height = vec![0,1,0,2,1,0,1,3,2,1,2,1];
        let result = Solution::trap(height);
        println!("{}", result);
    }

    #[test]
    fn test4() {
        let height = vec![0];
        let result = Solution::trap(height);
        println!("{}", result);
    }

    #[test]
    fn test5() {
        let height = vec![5,4,1,2];
        let result = Solution::trap(height);
        println!("{}", result); // excepted 1
    }

    #[test]
    fn test6() {
        let height = vec![5,5,4,7,8,2,6,9,4,5];
        let result = Solution::trap(height);
        println!("{}", result); // excepted 10
    }

    #[test]
    fn test11() {
        let height = vec![0,1,0,2,1,0,1,3,2,1,2,1];
        let result = Solution::trap1(height);
        println!("{}", result);
    }

    #[test]
    fn test21() {
        let height = vec![4,2,0,3,2,5];
        let result = Solution::trap1(height);
        println!("{}", result);
    }

    #[test]
    fn test31() {
        let height = vec![0,1,0,2,1,0,1,3,2,1,2,1];
        let result = Solution::trap1(height);
        println!("{}", result);
    }

    #[test]
    fn test41() {
        let height = vec![0];
        let result = Solution::trap1(height);
        println!("{}", result);
    }

    #[test]
    fn test51() {
        let height = vec![5,4,1,2];
        let result = Solution::trap1(height);
        println!("{}", result); // excepted 1
    }

    #[test]
    fn test61() {
        let height = vec![5,5,4,7,8,2,6,9,4,5];
        let result = Solution::trap1(height);
        println!("{}", result); // excepted 10
    }

    #[test]
    fn test_sum1() {
        let height = vec![0,1,0,2,1,0,1,3,2,1,2,1];
        let result = Solution::sum(&height, 3, height.len()-1);
        println!("{}", result);
    }

    #[test]
    fn test_sum2() {
        let height = vec![0,1,0,2,1,0,1,3,2,1,2,1];
        let result = Solution::sum(&height, 3, 4);
        println!("{}", result);
    }

    #[test]
    fn test_sum3() {
        let height = vec![0,1,0,2,1,0,1,3,2,1,2,1];
        let result = Solution::sum(&height, 2, 5);
        println!("{}", result);
    }

}

fn main() {
    println!("Hello, world!");
}