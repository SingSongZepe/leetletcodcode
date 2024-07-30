struct Solution;

use std::cmp::max;
use std::cmp::min;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l: u32 = 0;
        let mut r: u32 = (height.len() - 1) as u32;
        let mut m: i32 = Solution::area(height[l as usize], height[r as usize], (r-l) as i32);

        while l < r {
            if height[l as usize] < height[r as usize] {
                l += 1;
            } else {
                r -= 1;
            }
            m = max(m, Solution::area(height[l as usize], height[r as usize], (r-l) as i32));
        }

        m
    }
    pub fn area(h1: i32, h2: i32, l: i32) -> i32 {
        if h1 > h2 {
            l * h2
        } else {
            l * h1
        }
    }

    pub fn max_area1(height: Vec<i32>) -> i32 {
        let mut result = 0;
        // with idx iteration
        let mut iter = height.iter().enumerate();

        let mut p1 = iter.next();
        let mut p2 = iter.next_back();

        while let (Some((i, h1)), Some((j, h2))) = (p1, p2) {
            result = result.max((j-i) as i32 * min(h1,h2));
            if h1 < h2 {
                p1 = iter.next();
            } else {
                p2 = iter.next_back();
            }
            // (p1, p2) = (if h1 < h2 {iter.next()} else {p1}, if h1 < h2 {iter.next_back()} else {p2});
        }

        result
    }
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn test1() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        println!("t1 {}", Solution::max_area(height));
    }

    // #[test]
    fn test2() {
        let height = vec![1, 1];
        println!("t2 {}", Solution::max_area(height));
    }

    // #[test]
    fn test3() {
        let height = vec![1, 2, 1];
        println!("t3 {}", Solution::max_area(height));
    }

    // #[test]
    fn test4() {
        let height = vec![1, 2, 4, 3];
        println!("t4 {}", Solution::max_area(height));
    }

    // #[test]
    fn test5() {
        let height = vec![1,3,2,5,25,24,5];
        println!("t5 {}", Solution::max_area(height)); // excepted 24
    }

    #[test]
    fn test6() {
        let height = vec![1,3,2,5,25,24,5];
        println!("t6 {}", Solution::max_area1(height)); // excepted 24
    }
}