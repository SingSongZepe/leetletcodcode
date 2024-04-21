
struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let len1 = haystack.len();
        let len2 = needle.len();
        if len2 > len1 {
            return -1;
        }
        for i in 0..=len1-len2 {
            if &haystack[i..i+len2] == &needle[..] {
                return i as i32;
            }
        }
        -1
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
        let mut haystack = "sadbutsad".to_string();
        let mut needle = "sad".to_string();
        let idx = Solution::str_str(haystack, needle);
        println!("{}", idx);
    }

    #[test]
    fn test2() {
        let mut haystack = "leetcode".to_string();
        let mut needle = "leeto".to_string();
        let idx = Solution::str_str(haystack, needle);
        println!("{}", idx);
    }

    #[test]
    fn test3() {
        let mut haystack = "".to_string();
        let mut needle = "".to_string();
        let idx = Solution::str_str(haystack, needle);
        println!("{}", idx);
    }

    #[test]
    fn test4() {
        let mut haystack = "abs".to_string();
        let mut needle = "b".to_string();
        let idx = Solution::str_str(haystack, needle);
        println!("{}", idx);
    }

    #[test]
    fn test5() {
        let mut haystack = "a".to_string();
        let mut needle = "a".to_string();
        let idx = Solution::str_str(haystack, needle);
        println!("{}", idx);
    }

    #[test]
    fn test6() {
        let mut haystack = "aaa".to_string();
        let mut needle = "aaaa".to_string();
        let idx = Solution::str_str(haystack, needle);
        println!("{}", idx);
    }
}


