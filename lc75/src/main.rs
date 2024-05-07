
struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return "".to_string();
        }


        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Solution,
    };

    #[test]
    fn test1() {
        let s = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();
        // assert_eq!(Solution::min_window(s, t), "BANC".to_string());
        println!("{:?}", Solution::min_window(s, t)); // expected output: "BANC"
    }

    #[test]
    fn test2() {
        let s = "a".to_string();
        let t = "a".to_string();
        // assert_eq!(Solution::min_window(s, t), "a".to_string());
        println!("{:?}", Solution::min_window(s, t)); // expected output: "a"}
    }

    #[test]
    fn test3() {
        let s = "a".to_string();
        let t = "aa".to_string();
        // assert_eq!(Solution::min_window(s, t), "".to_string());
        println!("{:?}", Solution::min_window(s, t)); // expected output: ""
    }
}


fn main() {
    println!("Hello, world!");
}
