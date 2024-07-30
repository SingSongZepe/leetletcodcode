struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut t = x;
        let mut sum: i64 = 0;
        // let mut sum: i32 = if x > 0 {
        //     0
        // } else {
        //     -0
        // };

        while t != 0 {
            sum = 10 * sum + (t % 10) as i64;
            t /= 10;
        }

        if sum > i32::MAX as i64 || sum < i32::MIN as i64 {
            return 0;
        } else {
            return sum as i32;
        }
    }
    pub fn reverse1(x: i32) -> i32 {
        x.abs()
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>() // there you must know the default if parse failed is 0 for i32 type
            .unwrap_or_default()
            * if x.is_negative() { -1 } else { 1 }
    }
}

fn main() {
    println!("{}", Solution::reverse1(1534236469));
}
