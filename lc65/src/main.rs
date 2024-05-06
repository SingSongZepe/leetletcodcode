struct Solution;

use regex::Regex;
impl Solution {
    pub fn is_number(s: String) -> bool {
        let re = Regex::new(r"^[-+]?(\d+(\.\d*)?|\.\d+)([eE][-+]?\d+)?$").unwrap();
        re.is_match(&s)
    }
}

enum CheckState {
    FloatSign, // ACCEPTS: '+' | '-' | '.' | '0'..='9'
    FloatInit, // ACCEPTS: '.' | '0'..='9'
    FloatNum,  // ACCEPTS: '.' | 'e' | 'E' | '0'..='9'
    IntInit,   // ACCEPTS: '0'..='9'
    IntNum,    // ACCEPTS: 'e' | 'E' | '0'..='9'
    ExpSign,   // ACCEPTS: '+' | '-' | '0'..='9'
    ExpInit,   // ACCEPTS: '0'..='9'
    ExpNum,    // ACCEPTS: '0'..='9'
}

impl CheckState {
    pub fn accept(&self, c: char) -> Result<Self, ()> {
        match (c, self) {
            ('+' | '-', Self::FloatSign) => Ok(Self::FloatInit),
            ('+' | '-', Self::ExpSign) => Ok(Self::ExpInit),
            ('.', Self::FloatSign | Self::FloatInit) => Ok(Self::IntInit),
            ('.', Self::FloatNum) => Ok(Self::IntNum),
            ('e' | 'E', Self::FloatNum | Self::IntNum) => Ok(Self::ExpSign),
            ('0'..='9', Self::FloatSign | Self::FloatInit | Self::FloatNum) => Ok(Self::FloatNum),
            ('0'..='9', Self::IntInit | Self::IntNum) => Ok(Self::IntNum),
            ('0'..='9', Self::ExpSign | Self::ExpInit | Self::ExpNum) => Ok(Self::ExpNum),
            _ => Err(()),
        }
    }

    pub fn is_valid_end_state(&self) -> bool {
        matches!(self, Self::FloatNum | Self::IntNum | Self::ExpNum)
    }
}

impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut state = CheckState::FloatSign;
        for c in s.chars() {
            state = match state.accept(c) {
                Ok(new_state) => new_state,
                Err(()) => return false,
            };
        }
        state.is_valid_end_state()
    }
}

// my solution

enum Status {
    Start,        // 0 start state
    Sign,         // 1 sign (+/-)
    SignNumber,   // 2 sign (+/-) followed by a number
    Dot,          // 3 decimal point
    DotNumber,    // 4 decimal point followed by a number
    Exp,          // 5 exponent marker (e/E)
    ExpSign,      // 6 exponent marker (+/-)
    ExpNumber,    // 7 exponent marker (+/-) followed by a number
    DotBeforeDigit, // 8 decimal point before a digit
}

impl Status {
    fn transition(&mut self, c: char) -> Result<Status, ()> {
        match self {
            Status::Start => {  // 0 start state
                match c {
                    '+' | '-' => Ok(Status::Sign),
                    '0'..='9' => Ok(Status::SignNumber),
                    '.' => Ok(Status::DotBeforeDigit),
                    _ => Err(())
                }
            },
            Status::Sign => { // 1 sign (+/-)
                match c {
                    '0'..='9' => Ok(Status::SignNumber),
                    '.' => Ok(Status::DotBeforeDigit),
                    _ => Err(())
                }
            },
            Status::SignNumber => { // 2 sign (+/-) followed by a number
                match c {
                    '0'..='9' => Ok(Status::SignNumber),
                    '.' => Ok(Status::Dot),
                    'e' | 'E' => Ok(Status::Exp),
                    _ => Err(())
                }
            },
            Status::Dot => { // 3 decimal point
                match c {
                    '0'..='9' => Ok(Status::DotNumber),
                    'e' | 'E' => Ok(Status::Exp),
                    _ => Err(())
                }
            },
            Status::DotNumber => { // 4 decimal point followed by a number
                match c {
                    '0'..='9' => Ok(Status::DotNumber),
                    'e' | 'E' => Ok(Status::Exp),
                    _ => Err(())
                }
            },
            Status::Exp => { // 5 exponent marker (e/E)
                match c {
                    '+' | '-' => Ok(Status::ExpSign),
                    '0'..='9' => Ok(Status::ExpNumber),
                    _ => Err(())
                }
            },
            Status::ExpSign => { // 6 exponent marker (+/-)
                match c {
                    '0'..='9' => Ok(Status::ExpNumber),
                    _ => Err(())
                }
            },
            Status::ExpNumber => { // 7 exponent marker (+/-) followed by a number
                match c {
                    '0'..='9' => Ok(Status::ExpNumber),
                    _ => Err(())
                }
            },
            Status::DotBeforeDigit => { // 8 decimal point before a digit
                match c {
                    '0'..='9' => Ok(Status::DotNumber),
                    _ => Err(())
                }
            },
            _ => Err(())
        }
    }
}

struct Solution1;

impl Solution1 {
    pub fn is_number(s: String) -> bool {
        let mut status = Status::Start;
        for c in s.chars() {
            status = match status.transition(c) {
                Ok(new_status) => new_status,
                Err(_) => return false
            }
        }
        matches!(status, Status::DotNumber | Status::SignNumber | Status::ExpNumber | Status::Dot)
    }
}


fn helper(ss: Vec<&str>) -> Vec<String> {
    let mut result = Vec::new();
    for s in ss {
        result.push(s.to_string());
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::{
        Solution,
        helper
    };

    #[test]
    fn test1() {
        let s = "0".to_string();
        let result = Solution::is_number(s);
        println!("{}", result);
    }

    #[test]
    fn test2() {
        let s = "e".to_string();
        let result = Solution::is_number(s);
        println!("{}", result);
    }

    #[test]
    fn test3() {
        let s = ".".to_string();
        let result = Solution::is_number(s);
        println!("{}", result);
    }

    // all numbers
    #[test]
    fn test4() {
        let ss = helper(vec!["2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7", "+6e-1", "53.5e93", "-123.456e789"]);
        let mut results: Vec<bool> = Vec::new();
        for s in ss.clone() {
            let result = Solution::is_number(s);
            results.push(result);
        }
        for (idx, r) in results.iter().enumerate() {
            println!("{} - {}", ss[idx], *r); // excepted all true
        }
    }

    // all invalid numbers
    #[test]
    fn test5() {
        let ss = helper(vec!["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"]);
        let mut results: Vec<bool> = Vec::new();
        for s in ss.clone() {
            let result = Solution::is_number(s);
            results.push(result);
        }
        for (idx, r) in results.iter().enumerate() {
            println!("{} - {}", ss[idx], *r); // excepted all false
        }
    }
}

fn main() {
    println!("Hello, world!");
}

