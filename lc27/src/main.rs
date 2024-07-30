use std::ptr::null_mut;

struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut ns = vec![];

        for i in 0..nums.len() {
            if nums[i] != val {
                ns.push(nums[i]);
            }
        }

        std::mem::swap(nums, &mut ns);

        nums.len() as i32
    }

    pub fn remove_element1(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&x| x != val);
        nums.len() as i32
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
        let mut v = vec![3, 2, 2, 3];
        let val = 3;
        let k = Solution::remove_element(&mut v, val);

        println!("{v:?}");
        println!("{}", k);
    }

    #[test]
    fn test2() {
        let mut v = vec![3];
        let val = 3;
        let k = Solution::remove_element(&mut v, val);

        println!("{v:?}");
        println!("{}", k);
    }

    #[test]
    fn test3() {
        let mut v = vec![0,1,2,2,3,0,4,2];
        let val = 2;
        let k = Solution::remove_element(&mut v, val);

        println!("{v:?}");
        println!("{}", k);
    }

    #[test]
    fn test4() {
        let mut v = vec![3, 2, 2, 3];
        let val = 3;
        let k = Solution::remove_element1(&mut v, val);

        println!("{v:?}");
        println!("{}", k);
    }

    #[test]
    fn test5() {
        let mut v = vec![3];
        let val = 3;
        let k = Solution::remove_element1(&mut v, val);

        println!("{v:?}");
        println!("{}", k);
    }

    #[test]
    fn test6() {
        let mut v = vec![0,1,2,2,3,0,4,2];
        let val = 2;
        let k = Solution::remove_element1(&mut v, val);

        println!("{v:?}");
        println!("{}", k);
    }
}
