struct Solution;

impl Solution {
    // good memory usage
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return  String::from("");
        }

        let mut prefix = strs[0].clone();

        for s in &strs[1..] {
            let mut i = 0;
            while i < prefix.len() && i < s.len() {
                if prefix.chars().nth(i) != s.chars().nth(i) {
                    break;
                }
                i += 1;
            }
            prefix = prefix[..i].to_string();
            if prefix.len() == 0 {
                return String::from("");
            }
        }

        prefix
    }

    //
    pub fn longest_common_prefix1(strs: Vec<String>) -> String {
        // if strs.is_empty() {
        //     return String::new();
        // }

        let mut prefix = strs[0].as_str();

        for s in &strs[1..] {
            let mut i = 0;
            let s_chars = s.chars();
            for (ch1, ch2) in prefix.chars().zip(s_chars) {
                if ch1 != ch2 {
                    break;
                }
                i += 1;
            }
            prefix = &prefix[..i];
            if prefix.len() == 0 {
                return String::new();
            }
        }

        prefix.to_string()
    }

    // beats 100% at speed 55% at memory
    pub fn longest_common_prefix2(strs: Vec<String>) -> String {
        let mut prefix = strs[0].as_str();
        for s in &strs[1..] {
            let mut i = 0;
            for (ch1, ch2) in prefix.chars().zip(s.chars()) {
                if ch1 != ch2 {
                    break;
                }
                i += 1;
            }
            prefix = &prefix[..i];

        }
        prefix.to_string()
    }

    // pub fn longest_common_prefix3(mut strs: Vec<String>) -> String {
    //     // let mut prefix = strs[0].as_str();
    //     for s in &mut strs[1..] {
    //         let mut i = 0;
    //         for (ch1, ch2) in strs[0].as_str().chars().zip(s.chars()) {
    //             if ch1 != ch2 {
    //                 break;
    //             }
    //             i += 1;
    //         }
    //         strs[0] = (&strs[0][..i]).to_string();
    //         if prefix.len() == 0 {
    //             return prefix.to_string();
    //         }
    //     }
    //     strs[0].clone()
    // }
}

fn main() {
    println!("Hello, world!");
}

fn helper(v: Vec<&str>) -> Vec<String> {
    let mut v_string: Vec<String> = vec![];
    for s in v {
        v_string.push(String::from(s));
    }

    v_string
}

#[cfg(test)]
mod tests {
    use crate::{helper, Solution};

    #[test]
    fn test1() {
        let v: Vec<String> = helper(vec!["flower", "flow", "flight"]);
        println!("t1 {}", Solution::longest_common_prefix(v));
    }

    #[test]
    fn test2() {
        let v: Vec<String> = helper(vec!["dog", "racecar", "car"]);
        println!("t2 {}", Solution::longest_common_prefix(v));
    }

    #[test]
    fn test3() {
        let v: Vec<String> = helper(vec!["flower", "flow", "flight"]);
        println!("t3 {}", Solution::longest_common_prefix1(v));
    }

    #[test]
    fn test4() {
        let v: Vec<String> = helper(vec!["dog", "racecar", "car"]);
        println!("t4 {}", Solution::longest_common_prefix1(v));
    }

    #[test]
    fn test5() {
        let v: Vec<String> = helper(vec!["flower", "flow", "flight"]);
        println!("t5 {}", Solution::longest_common_prefix2(v));
    }

    #[test]
    fn test6() {
        let v: Vec<String> = helper(vec!["dog", "racecar", "car"]);
        println!("t6 {}", Solution::longest_common_prefix2(v));
    }

    #[test]
    fn test7() {
        let v: Vec<String> = helper(vec!["flower", "flow", "flight"]);
        println!("t7 {}", Solution::longest_common_prefix2(v));
    }

    #[test]
    fn test8() {
        let v: Vec<String> = helper(vec!["dog", "racecar", "car"]);
        println!("t8 {}", Solution::longest_common_prefix2(v));
    }
}
