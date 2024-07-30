
struct Solution;

impl Solution {
    // dutch national flag algorithm
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut l, mut m, mut r): (usize, usize, i32) = (0, 0, nums.len() as i32 - 1);
        while m as i32 <= r {
            match nums[m] {
                0 => {
                    nums.swap(l, m);
                    l += 1;
                    m += 1;
                },
                1 => {
                    m += 1;
                },
                2 => {
                    nums.swap(m,  r as usize);
                    r -= 1;
                },
                _ => {
                    panic!("Invalid color value");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Solution,
    };

    #[test]
    fn test1() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums);
        // assert_eq!(nums, [0, 0, 1, 1, 2, 2]);
        println!("{:?}", nums); // expected [0, 0, 1, 1, 2, 2]
    }

    #[test]
    fn test2() {
        let mut nums = vec![2, 0, 1];
        Solution::sort_colors(&mut nums);
        // assert_eq!(nums, [0, 1, 2]);
        println!("{:?}", nums); // expected [0, 1, 2]
    }

    #[test]
    fn test3() {
        let mut nums = vec![2];
        Solution::sort_colors(&mut nums);
        // assert_eq!(nums, [0, 1, 2]);
        println!("{:?}", nums); // expected [0, 1, 2]
    }
}


fn main() {
    println!("Hello, world!");
}
