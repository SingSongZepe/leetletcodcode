use std::collections::HashMap;

struct Solution;

impl Solution {
    // bad implementation
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return "".to_string();
        }
        let s = s.as_bytes();
        let t = t.as_bytes();

        // two pointers
        let (mut l, mut r): (usize, usize) = (0, 0);

        let len = s.len();
        // cmp
        let mut curr_count = 0;

        // record result
        let (mut from, mut to): (usize, usize) = (0, len-1);

        let mut hm = HashMap::<u8, usize>::new();
        for u in t {
            let count = hm.entry(*u).or_insert(0);
            *count += 1;
        }
        let mut hmt = hm.clone();

        while r != len {
            let mut found = false;
            if let Some(rcount) = hmt.get_mut(&s[r]) {
                if *rcount > 0 {
                    *rcount -= 1;
                    curr_count += 1;
                    found = true;
                }
            }
            if found && curr_count == t.len() {
                if to-from > r-l {
                    from = l;
                    to = r;
                }
                hmt = hm.clone();
                curr_count = 0;
            }
            r += 1;
            if !found {
                l = r;
            }
        }

        // println!("{:?}", s[from..=to]);
        "".to_string()
    }

    // fn make_hm()
}

struct Solution1;

impl Solution1 {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return "".to_string();
        }
        let     sb     = s.as_bytes();
        let mut i      = 0;
        let mut j      = 0;
        let mut tlen   = t.len();
        let mut counts = [0; (b'z' - b'A') as usize + 1];
        let mut win    = None;

        macro_rules! counts {
            [$i:expr] =>
            (counts[($i - b'A') as usize])
        }

        t.bytes().for_each(|b| counts![b] += 1);

        while j < s.len() {
            if counts![sb[j]] > 0 {
                tlen -= 1;
            }
            counts![sb[j]] -= 1;
            j += 1;
            while tlen == 0 {
                if j - i < win.map_or(usize::MAX, |(i, j)| j - i) {
                    win = Some((i, j));
                }
                counts![sb[i]] += 1;
                if counts![sb[i]] > 0 {
                    tlen += 1;
                }
                i += 1;
            }
        }
        win.map_or("".into(), |(i, j)| s[i..j].into())
    }
}


struct Solution2;

impl Solution2 {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return "".to_string();
        }
        let sb = s.as_bytes();
        let ls = sb.len();
        let mut lr = None;
        let (mut l, mut r) = (0, 0);
        let mut counts = [0; (b'z' - b'A') as usize + 1];
        let mut remaining = t.len();

        macro_rules! counts {
            [$i:expr] =>
            (counts[($i - b'A') as usize])
        }
        t.bytes().for_each(|b| counts![b] += 1);

        while r < ls {
            if counts![sb[r]] > 0 {
                remaining -= 1;
            }
            counts![sb[r]] -= 1;
            r += 1;
            while remaining == 0 {
                if r - l < lr.map_or(usize::MAX, |(i, j)| j - i) {
                    lr = Some((l, r))
                }
                if lr.map_or(false, |(i, j)| j - i == t.len()) {
                    return s[lr.unwrap().0..lr.unwrap().1].to_string();
                }
                counts![sb[l]] += 1;
                if counts![sb[l]] > 0 {
                    remaining += 1;
                }
                l += 1;
            }
        }
        lr.map_or("".into(), |(i, j)| s[i..j].into())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Solution,
        Solution1,
        Solution2,
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

    #[test]
    fn test11() {
        let s = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();
        // assert_eq!(Solution::min_window(s, t), "BANC".to_string());
        println!("{:?}", Solution1::min_window(s, t)); // expected output: "BANC"
    }

    #[test]
    fn test21() {
        let s = "a".to_string();
        let t = "a".to_string();
        // assert_eq!(Solution::min_window(s, t), "a".to_string());
        println!("{:?}", Solution1::min_window(s, t)); // expected output: "a"}
    }

    #[test]
    fn test31() {
        let s = "a".to_string();
        let t = "aa".to_string();
        // assert_eq!(Solution::min_window(s, t), "".to_string());
        println!("{:?}", Solution1::min_window(s, t)); // expected output: ""
    }

    #[test]
    fn test12() {
        let s = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();
        // assert_eq!(Solution::min_window(s, t), "BANC".to_string());
        println!("{:?}", Solution2::min_window(s, t)); // expected output: "BANC"
    }

    #[test]
    fn test22() {
        let s = "a".to_string();
        let t = "a".to_string();
        // assert_eq!(Solution::min_window(s, t), "a".to_string());
        println!("{:?}", Solution2::min_window(s, t)); // expected output: "a"}
    }

    #[test]
    fn test32() {
        let s = "a".to_string();
        let t = "aa".to_string();
        // assert_eq!(Solution::min_window(s, t), "".to_string());
        println!("{:?}", Solution2::min_window(s, t)); // expected output: ""
    }

    #[test]
    fn test42() {
        let s = "a".to_string();
        let t = "b".to_string();
        // assert_eq!(Solution::min_window(s, t), "".to_string());
        println!("{:?}", Solution2::min_window(s, t)); // expected output: ""
    }

    #[test]
    fn test52() {
        let s = "ccccabccccc".to_string();
        let t = "abc".to_string();
        // assert_eq!(Solution::min_window(s, t), "".to_string());
        println!("{:?}", Solution2::min_window(s, t)); // expected output: ""
    }

    #[test]
    fn test_map_or() {
        let maybe_number: Option<i32> = Some(42);
        let result = maybe_number.map_or(-1, |num| num * 2);

        println!("{}", result); // Output: 84

        let maybe_none: Option<i32> = None;
        let result2 = maybe_none.map_or(-1, |num| num * 2);

        println!("{}", result2); // Output: -1

        let v = vec![Some(1), None, Some(3), None, Some(5)];
        // v.iter().map
    }

    #[test]
    fn test_into() {
        // into implement
        // impl<T, U> const Into<U> for T //! T can be transformed into U
        //     where
        //         U: ~const From<T>,     //! forced to check whether U implements trait From<T>
        // {
        //     /// Calls `U::from(self)`.
        //     ///
        //     /// That is, this conversion is whatever the implementation of
        //     /// <code>[From]&lt;T&gt; for U</code> chooses to do.
        //     fn into(self) -> U {
        //         U::from(self)
        //     }
        // }
        let s: String = "".into();
        println!("{:?}", s);
    }
}


fn main() {
    println!("Hello, world!");
}
