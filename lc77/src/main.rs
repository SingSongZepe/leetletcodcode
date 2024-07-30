
struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];; // C(n, k) kind of number of combinations
        let mut select = vec![];
        Solution::combine_recursive(n, k, 1, &mut select, &mut result);
        result
    }
    pub fn combine_recursive(n: i32, k: i32, start: i32, select: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if k == 0 {
            result.push(select.clone());
        }
        for i in start..=n {
            select.push(i);
            Solution::combine_recursive(n, k - 1, i + 1, select, result);
            select.pop();
        }
    }
    pub fn combination(n: i32, k: i32) -> usize {
        let (mut fac1, mut fac2, mut fac3) = (1, 1, 1);
        if n - k > k {
            for i in 1..=k {
                fac1 *= i;
            }
            fac2 = fac1;
            for i in k+1..=n-k {
                fac2 *= i;
            }
            fac3 = fac2;
            for i in n-k+1..=n {
                fac3 *= i;
            }
        } else {
            for i in 1..=n-k {
                fac1 *= i;
            }
            fac2 = fac1;
            for i in n-k+1..=k {
                fac2 *= i;
            }
            fac3 = fac2;
            for i in k+1..=n {
                fac3 *= i;
            }
        }
        (fac3 / (fac1 * fac2)) as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Solution,
    };

    #[test]
    fn test1() {
        let n = 4;
        let k = 2;
        let result = Solution::combine(n, k);
        println!("{:?}", result);
    }

    #[test]
    fn test2() {
        let n = 1;
        let k = 1;
        let result = Solution::combine(n, k);
        println!("{:?}", result);
    }

    #[test]
    fn test_combination1() {
        let n = 4;
        let k = 2;
        let result = Solution::combination(n, k);
        println!("{}", result);
    }

    #[test]
    fn test_combination2() {
        let n = 1;
        let k = 1;
        let result = Solution::combination(n, k);
        println!("{}", result);
    }

    #[test]
    fn test_combination3() {
        let n = 2;
        let k = 1;
        let result = Solution::combination(n, k);
        println!("{}", result);
    }

    #[test]
    fn test_combination4() {
        let n = 5;
        let k = 2;
        let result = Solution::combination(n, k);
        println!("{}", result);
    }
}

fn main() {
    println!("Hello, world!");
}
