struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut res = String::from("");

        let mut x = num;
        while x >= 1000 {
            res.push('M');
            x -= 1000;
        }

        if x >= 900 {
            res.push_str("CM");
            x -= 900;
        }
        if x >= 500 {
           res.push('D');
            x -= 500;
        }
        if x >= 400 {
            res.push_str("CD");
            x -= 400;
        }
        while x >= 100 {
            res.push('C');
            x -= 100;
        }

        if x >= 90 {
            res.push_str("XC");
            x -= 90;
        }
        if x >= 50 {
            res.push('L');
            x -= 50;
        }
        if x >= 40 {
            res.push_str("XL");
            x -= 40;
        }
        while x >= 10 {
            res.push('X');
            x -= 10;
        }

        if x == 9 {
            res.push_str("IX");
            return res;
        }
        if x >= 5 {
            res.push('V');
            x -= 5;
        }
        if x == 4 {
            res.push_str("IV");
            return res;
        }
        while x >= 1 {
            res.push('I');
            x -= 1;
        }

        res
    }

    pub fn int_to_roman1(mut n: i32) -> String {
        let mut result = String::new();
        // let mut n: i16 = num as i16;

        let roman_lib: Vec<(i32, &str)> = vec![
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];

        for (value, symbol) in roman_lib {
            while n >= value {
                result.push_str(symbol);
                n -= value;
            }
        }

        result
    }

    pub fn int_to_roman2(num: i32) -> String {
        let mut result = String::new();
        let mut n: i16 = num as i16;

        let roman_lib: Vec<(i16, &str)> = vec![
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
        ];

        for (value, symbol) in roman_lib {
            while n >= value {
                result.push_str(symbol);
                n -= value;
            }
        }

        // for
        // (9, "IX"),
        // (5, "V"),
        // (4, "IV"),
        // (1, "I"),

        if n == 9 {
            result.push_str("IX");
            return result;
        }
        if n >= 5 {
            result.push_str("V");
            n -= 5;
        }
        if n == 4 {
            result.push_str("IV");
            return  result;
        }
        while n >= 1 {
            result.push_str("I");
            n -= 1;
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
    // fn test1() {
    //     let num = 58;
    //     println!("t1 {}", Solution::int_to_roman(num));
    // }
    //
    // #[test]
    // fn test2() {
    //     let num = 3;
    //     println!("t2 {}", Solution::int_to_roman(num));
    // }
    //
    // #[test]
    // fn test3() {
    //     let num = 1994;
    //     println!("t3 {}", Solution::int_to_roman(num));
    // }
    //
    //
    // #[test]
    // fn test4() {
    //     let num = 899;
    //     println!("t4 {}", Solution::int_to_roman(num));
    // }


    #[test]
    fn test1() {
        let num = 58;
        println!("t1 {}", Solution::int_to_roman1(num));
    }

    #[test]
    fn test2() {
        let num = 3;
        println!("t2 {}", Solution::int_to_roman1(num));
    }

    #[test]
    fn test3() {
        let num = 1994;
        println!("t3 {}", Solution::int_to_roman1(num));
    }

    #[test]
    fn test4() {
        let num = 899;
        println!("t4 {}", Solution::int_to_roman1(num));
    }
}
