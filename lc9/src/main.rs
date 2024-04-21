struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        } else if x < 10 {
            return true;
        }

        if let Some(ch) = x.to_string().chars().next() {
            if ch as i32 - 48 == x % 10 {
                let num_str = x.to_string();
                return Solution::is_p(String::from(&num_str[1..num_str.len()-1]));
            }
        }
        false
    }
    pub fn is_p(s: String) -> bool {
        if s.is_empty() || s.len() == 1 {
            return true;
        }
        if let Some(ch) = s.chars().next() {
            if let Ok(num) = s.parse::<i32>() {
                if ch as i32 - 48 == num % 10 {
                    return Solution::is_p(String::from(&s[1..s.len()-1]));
                }
            }
        }
        false
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn test1() {
        let number = 100;
        if Solution::is_palindrome(number) {
            print!("t1 palindrome number");
        } else {
            print!("t1 not palindrome number");
        }
        println!();
    }

    // #[test]
    fn test2() {
        let number = -101;
        if Solution::is_palindrome(number) {
            print!("t2 palindrome number");
        } else {
            print!("t2 not palindrome number");
        }
        println!();
    }

    // #[test]
    fn test3() {
        let number = 101;
        if Solution::is_palindrome(number) {
            print!("t3 palindrome number");
        } else {
            print!("t3 not palindrome number");
        }
        println!();
    }

    // #[test]
    fn test4() {
        let number = -2147447412;
        if Solution::is_palindrome(number) {
            print!("t4 palindrome number");
        } else {
            print!("t4 not palindrome number");
        }
        println!();
    }

    // #[test]
    fn test5() {
        let number = 11;
        if Solution::is_palindrome(number) {
            print!("t5 palindrome number");
        } else {
            print!("t5 not palindrome number");
        }
        println!();
    }

    #[test]
    fn test6() {
        let number = 1000021;
        if Solution::is_palindrome(number) {
            print!("t6 palindrome number");
        } else {
            print!("t6 not palindrome number");
        }
        println!();
    }

    // #[test]
    fn test() {
        let num: i32 = 32090;
        let num_str = num.to_string();
        let middle_num: i32 = (&num_str[1..num_str.len()-1]).parse().unwrap();
        println!("{}", middle_num);
    }
}
