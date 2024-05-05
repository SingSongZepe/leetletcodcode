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
