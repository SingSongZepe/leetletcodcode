
struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut memo = vec![0; n as usize + 1];
        Solution::climb(n, n, &mut memo)
    }
    pub fn climb(curr: i32, n: i32, memo: &mut Vec<i32>) -> i32 {
        if curr == 1 || curr == 0 {
            return 1;
        }
        if memo[curr as usize] != 0 {
            return memo[curr as usize];
        }
        memo[curr as usize] = Solution::climb(curr - 1, n, memo) + Solution::climb(curr - 2, n, memo);
        memo[curr as usize]
    }
}

fn factorial(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    (1..=n).product()
}

fn combination(n: u64, k: u64) -> u64 {
    factorial(n) / (factorial(k) * factorial(n - k))
}

fn calculate_result(n: u64) -> u64 {
    if n % 2 == 1 {
        let start = (n + 1) / 2;
        let end = (n - 1) / 2;
        (0..=end).map(|i| combination(start + 2 * i, 1 + 2 * i)).sum()
    } else {
        let start = n / 2;
        let end = n / 2;
        (0..=end).map(|i| {
            combination(start + 2 * i, 2 * i)
        }).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Solution,
        calculate_result,
        combination,
        factorial,
    };

    #[test]
    fn test1() {
        let n = 2;
        let result = Solution::climb_stairs(n);
        println!("{}", result);
    }

    #[test]
    fn test2() {
        let n = 3;
        let result = Solution::climb_stairs(n);
        println!("{}", result);
    }

    #[test]
    fn test3() {
        let n = 6;
        let result = Solution::climb_stairs(n);
        println!("{}", result);
    }

    #[test]
    fn test4() {
        let n = 44;
        let result = Solution::climb_stairs(n);
        println!("{}", result);
    }

    #[test]
    fn test_calulate_result() {
        let n = 2;
        let result = calculate_result(n);
        println!("{}", result);
    }

    #[test]
    fn test_combination1() {
        let n = 5;
        let m = 2;
        let result = combination(n, m);
        println!("{}", result);
    }

    #[test]
    fn test_combination2() {
        let n = 5;
        let m = 0;
        let result = combination(n, m);
        println!("{}", result);
    }

    #[test]
    fn test_combination3() {
        let n = 5;
        let m = 1;
        let result = combination(n, m);
        println!("{}", result);
    }

    #[test]
    fn test_factorial1() {
        let n = 10;
        let result = factorial(n);
        println!("{}", result);
    }
}

fn main() {
    println!("Hello, world!");
}
