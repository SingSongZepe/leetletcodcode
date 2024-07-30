struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let str = s.trim_start();
        let mut negative = false;
        let mut sum: i32 = 0;
        if let Some(ch) = str.chars().next() {
            if ch == '-' {
                negative = true;
            } else if ch == '+' {
            } else if ch.is_numeric() {
                sum = (ch as usize - 48) as i32;
            } else {
                return 0;
            }
        }

        for i in 1..str.len() {
            if let Some(ch) = str.chars().nth(i) {
                if ch.is_numeric() {
                    match 10_i32.checked_mul(sum).and_then(|x| x.checked_add((ch as i32) - 48)) {
                        Some(res) => sum = res,
                        None => {
                            if negative {
                                return i32::MIN;
                            } else {
                                return i32::MAX;
                            }
                        }
                    }
                    continue;
                }
            }
            break;
        }

        if negative {
            -sum as i32

        } else {
            sum as i32

        }
    }
}

fn main() {
    println!("Hello World");
    // Solution::my_atoi(String::from(" -45"));
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test1() {
    //     let str = String::from("-987 word");
    //     println!("{}", Solution::my_atoi(String::from(str)));
    // }

    #[test]
    fn test2() {
        let str = String::from("42");
        println!("t2 {}", Solution::my_atoi(String::from(str)));
    }

    #[test]
    fn test3() {
        let str = String::from("   -42");
        println!("t3 {}", Solution::my_atoi(String::from(str)));
    }

    #[test]
    fn test4() {
        let str = String::from("4193 with words");
        println!("t4 {}", Solution::my_atoi(String::from(str)));
    }

    #[test]
    fn test5() {
        let str = String::from("00000-42a1234");
        println!("t5 {}", Solution::my_atoi(String::from(str)));
    }

    #[test]
    fn test6() {
        let str = String::from("-000000000000000000000000000001");
        println!("t6 {}", Solution::my_atoi(String::from(str)));
    }

    #[test]
    fn test7() {
        let str = String::from("-91283472332");
        println!("t7 {}", Solution::my_atoi(String::from(str)));
    }

    #[test]
    fn test8() {
        let str = String::from("-115579378e25");
        println!("t8 {}", Solution::my_atoi(String::from(str)));
    }

    #[test]
    fn test9() {
        let str = String::from("9223372036854775808");
        println!("t9 {}", Solution::my_atoi(String::from(str)));
    }

    // #[test]
    fn test_2() {
        let ch: char = '6';
        println!("{}", (ch as usize - 48) as i32);
    }
}
