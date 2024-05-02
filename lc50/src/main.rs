
struct Solution;

// impl Solution {
//     pub fn my_pow(x: f64, n: i32) -> f64 {
//         if x == 1.0 {
//             return x;
//         }
//         if n == 0 {
//             1_f64
//         } else if n > 0 {
//             Solution::it(x, n)
//         } else {
//             1_f64 / Solution::it(x, -n)
//         }
//     }
//
//     pub fn it(x: f64, n: i32) -> f64 {
//         if n == 0 {
//             1.0
//         } else if n == 1 {
//             x
//         } else {
//             Solution::it(x * x, n / 2)
//         }
//     }
// }


impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        x
    }
}

impl Solution {
    pub fn my_pow1(x: f64, n: i32) -> f64 {
        let res = Self::recurse(x, (n as i64).abs());
        match n.cmp(&0) {
            std::cmp::Ordering::Greater => res,
            _ => 1.0 / res,
        }
    }

    fn recurse(m: f64, pow: i64) -> f64 {
        match pow {
            0 => 1.0,
            p => {
                let part = Self::recurse(m, p / 2);
                if p % 2 == 1 {
                    part * part * m
                } else {
                    part * part
                }
            }
        }
    }
}

impl Solution {
    pub fn my_pow2(x: f64, n: i32) -> f64 {
        match (x,n) {
            (_,0) => 1.0,
            (_,_) => {
                if n%2 == 0 {
                    Self::my_pow2(x, n/2) * Self::my_pow2(x, n/2)
                } else {
                    if n < 0 {
                        (1.0/x) * Self::my_pow2(x,n/2) * Self::my_pow2(x,n/2)
                    } else {
                        x * Self::my_pow2(x,n/2) * Self::my_pow2(x, n/2)
                    }
                }
            }
        }

    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let x = 2.0000_f64;
        let n = 10_i32;
        let result = Solution::my_pow(x, n);
        println!("{result}");
    }

    #[test]
    fn test2() {
        let x = 2.1000_f64;
        let n = 3_i32;
        let result = Solution::my_pow(x, n);
        println!("{result}");
    }

    #[test]
    fn test3() {
        let x = 2.0000_f64;
        let n = -2_i32;
        let result = Solution::my_pow(x, n);
        println!("{result}");
    }

    #[test]
    fn test11() {
        let x = 2.0000_f64;
        let n = 10_i32;
        let result = Solution::my_pow1(x, n);
        println!("{result}");
    }

    #[test]
    fn test21() {
        let x = 2.1000_f64;
        let n = 3_i32;
        let result = Solution::my_pow1(x, n);
        println!("{result}");
    }

    #[test]
    fn test31() {
        let x = 2.0000_f64;
        let n = -2_i32;
        let result = Solution::my_pow1(x, n);
        println!("{result}");
    }
}

fn main() {
    println!("Hello, world!");
}
