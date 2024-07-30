
struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let len = digits.len();
        digits[len-1] += 1;
        for i in (1..len).rev() {
            if digits[i] == 10 {
                digits[i] = 0;
                digits[i-1] += 1;
            } else {
                break;
            }
        }
        if digits[0] == 10 {
            digits[0] = 0;
            digits.insert(0, 1);
        }
        digits
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Solution
    };

    #[test]
    fn test1() {
        let digits = vec![1, 2, 3];
        let result = Solution::plus_one(digits);
        println!("{:?}", result);
    }

    #[test]
    fn test2() {
        let digits = vec![4, 3, 2, 1];
        let result = Solution::plus_one(digits);
        println!("{:?}", result);
    }

    #[test]
    fn test3() {
        let digits = vec![9];
        let result = Solution::plus_one(digits);
        println!("{:?}", result);
    }

    #[test]
    fn test4() {
        let digits = vec![9, 9, 9];
        let result = Solution::plus_one(digits);
        println!("{:?}", result);
    }
}

fn main() {
    println!("Hello, world!");
}
