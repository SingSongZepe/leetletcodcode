struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if let Some(i) = dividend.checked_div(divisor) {
            i
        } else {
            i32::MAX
        }
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
        let dividend = 10;
        let divisor = 3;
        let output = Solution::divide(dividend, divisor);
        println!("{}", output);
    }

    #[test]
    fn test2() {
        let dividend = 7;
        let divisor = -3;
        let output = Solution::divide(dividend, divisor);
        println!("{}", output);
    }

    #[test]
    fn test3() {
        let dividend = -2147483648;
        let divisor = -1;
        let output = Solution::divide(dividend, divisor);
        println!("{}", output);
    }

    #[test]
    fn test4() {
        let dividend = 2147483647;
        let divisor = -1;
        let output = Solution::divide(dividend, divisor);
        println!("{}", output);
    }


    #[test]
    fn test() {
        let i = 7 / 3;
        println!("{}", i);

        let dividend: i32 = -2147483648;
        let divisor: i32 = -1;
        if let Some(i) = dividend.checked_div(divisor) {
            println!("{}", i);
        }
    }
}