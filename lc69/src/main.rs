
struct Solution;

impl Solution {
    pub fn my_sqrt(mut x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let n = x;
        x = 46340.min(x);

        let mut decreased = false;
        loop {
            let nx = (x + n / x) >> 1 ;
            if nx == x || (nx > x && decreased) {
                break;
            }
            decreased = nx < x;
            x = nx;
        }
        x
    }
}

struct Solution1;

impl Solution1 {
    pub fn my_sqrt(number: f32) -> f32 {
        let x2 = number * 0.5;
        let y = number;
        let i: i32 = unsafe { std::mem::transmute(y) };  // evil floating point bit level hacking
        let mut y: f32;
        let three_halves = 1.5;

        let magic_number = 0x5f3759df;
        let mut i = magic_number - (i >> 1);  // What the fuck?
        y = unsafe { std::mem::transmute(i) };
        y = y * (three_halves - (x2 * y * y));  // 1st iteration
        // y = y * (three_halves - (x2 * y * y));   // 2nd iteration, this can be removed

        y
    }
}


// float Q_rsqrt( float number )
// {
// long i;
// float x2, y;
// const float threeHalfs = 1.5f;
//
// x2 = number * 0.5f;
// y  = number;
// i  = * ( long * ) &y;                       // evil floating point bit level hacking
// i  = 0x5f3759df - ( i >> 1 );               // What the fuck?
// y  = * ( float * ) &i;
// y  = y * ( threeHalfs - ( x2 * y * y ) );   // 1st iteration
// //	y  = y * ( threeHalfs - ( x2 * y * y ) );   // 2nd iteration, this can be removed
//
// return y;
// }


#[cfg(test)]
mod tests {
    use crate::{
        Solution,
        Solution1,
    };

    #[test]
    fn test0() {
        let x = 1;
        let result = Solution::my_sqrt(x);
        println!("{}", result); // expected output: 1
    }

    #[test]
    fn test1() {
        let x = 4;
        let result = Solution::my_sqrt(x);
        println!("{}", result); // expected output: 2
    }

    #[test]
    fn test2() {
        let x = 8;
        let result = Solution::my_sqrt(x);
        println!("{}", result); // expected output: 2
    }

    #[test]
    fn test3() {
        let x = 999;
        let result = Solution::my_sqrt(x);
        println!("{}", result); // expected output:
    }

    #[test]
    fn test4() {
        let x = 2147483647;
        let result = Solution::my_sqrt(x);
        println!("{}", result); // expected output: 46340
    }

    #[test]
    fn test01() {
        let x = 1;
        let result = Solution1::my_sqrt(x as f32) as i32;
        println!("{}", result); // expected output: 1
    }

    #[test]
    fn test21() {
        let x = 8;
        let result = Solution1::my_sqrt(x as f32);
        println!("{}", result); // expected output: 2
    }

    #[test]
    fn test31() {
        let x = 999;
        let result = Solution1::my_sqrt(x as f32);
        println!("{}", result); // expected output:
    }

    #[test]
    fn test41() {
        let x = 2147483647;
        let result = Solution::my_sqrt(x);
        println!("{}", result); // expected output: 46340
    }

    #[test]
    fn test_shift_right1() {
        let x = 0b111111;
        let result = x >> 1;
        println!("{result}");
    }

    #[test]
    fn test_shift_right2() {
        let x = i32::MAX;
        let result = x >> 1;
        println!("{result}");
    }

    #[test]
    fn test_shift_right3() {
        let x = 46340;
        let n = i32::MAX;
        let result = (n / x) >> 1;
        println!("{result}");
    }
}

fn main() {
    println!("Hello, world!");
}
