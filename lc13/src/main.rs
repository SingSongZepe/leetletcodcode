struct  Solution;

impl Solution {
    pub fn roman_to_int(mut s: String) -> i32 {
        let mut result = 0;
        let prefix = vec![
            ("M", 1000),
            ("CM", 900),
            ("D", 500),
            ("CD", 400),
            ("C", 100),
            ("XC", 90),
            ("L", 50),
            ("XL", 40),
            ("X", 10),
            ("IX", 9),
            ("V", 5),
            ("IV", 4),
            ("I", 1),
        ];

        for (pfix, value) in prefix {
            while s.starts_with(pfix) {
                result += value;
                s = String::from(&s[pfix.len()..]);
            }
        }

        result
    }


    // O(n)
    // when we place a "I" front X,
    pub fn get_val(p: &mut i32, val : i32) -> i32{
        if val >= *p {
            *p = val;
            return val;
        } else {
            *p = val;
            return -val;
        }
    }
    pub fn roman_to_int2(s: String) -> i32 {
        let mut prev: i32 = 0;
        let mut sum: i32=0;
        for c in s.chars().rev() {
            // do something with `c`
            match c {
                'I' => sum += Self::get_val(&mut prev, 1),
                'V' => sum += Self::get_val(&mut prev, 5),
                'X' => sum += Self::get_val(&mut prev, 10),
                'L' => sum += Self::get_val(&mut prev, 50),
                'C' => sum += Self::get_val(&mut prev, 100),
                'D' => sum += Self::get_val(&mut prev, 500),
                'M' => sum += Self::get_val(&mut prev, 1000),
                _ => panic!("panic"),
            }
        }
        sum
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
        let roman = String::from("DCCCXCIX");
        println!("t1 {}", Solution::roman_to_int(roman));  // 899
    }
    #[test]
    fn test2() {
        let roman = String::from("LVIII");
        println!("t2 {}", Solution::roman_to_int(roman));  // 58
    }
    #[test]
    fn test3() {
        let roman = String::from("III");
        println!("t3 {}", Solution::roman_to_int(roman)); // 3
    }
    #[test]
    fn test4() {
        let roman = String::from("MCMXCIV");
        println!("t4 {}", Solution::roman_to_int(roman)); // 1994
    }

    #[test]
    fn test5() {
        let roman = String::from("III");
        println!("t5 {}", Solution::roman_to_int2(roman)); // 3
    }
    #[test]
    fn test6() {
        let roman = String::from("MCMXCIV");
        println!("t6 {}", Solution::roman_to_int2(roman)); // 1994
    }

    // #[test]
    // fn test() {
    //     let mut s = "CDFD";
    //     let pfix = "CD";
    //     println!("{}", &s[..pfix.len()] == pfix);
    // }
}