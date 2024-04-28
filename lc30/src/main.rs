struct Solution;

use std::collections::HashMap;

type HM<'a> = std::collections::HashMap<&'a [u8], i32>;

impl Solution {
    // time exceed
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let len1 = s.len();
        let tlen2 = words.iter().fold(0, |x, s| x + s.len());
        if len1 < tlen2 {
            return vec![];
        }
        let mut results : Vec<i32> = vec![];

        for i in 0..=len1-tlen2 {
            if Solution::match_str(&s[i..i+tlen2], &words) {
                results.push(i as i32);
            }
        }

        results
    }
    // can't process
    // s = "refe"
    // word = ["e", "ref"]
    // pub fn match_str(s: &str, words: &Vec<String>) -> bool {
    //     // there the s.length = words.total_length
    //     let len = s.len();
    //     let mut used = vec![false; len];
    //     for word in words {
    //         let len2 = word.len();
    //         let mut found = false;
    //         for i in 0..=len-len2 {
    //             if used[i..i+len2] != vec![false; len2] { // all bit are not used
    //                 continue
    //             }
    //             if word[..] == s[i..i+len2] {
    //                 found = true;
    //                 for j in i..i+len2 {
    //                     used[j] = true;
    //                 }
    //                 break
    //             }
    //         }
    //         if !found {
    //             return  false;
    //         }
    //     }
    //     if used == vec![true; len] {
    //         true
    //     } else {
    //         false
    //     }
    // }
    pub fn match_str(s: &str, words: &Vec<String>) -> bool {
        if s.is_empty() {
            return true;
        }
        for (i, word) in words.iter().enumerate() {
            if s.starts_with(word) {
                let remaining = &s[word.len()..];
                // let filtered_words: Vec<String> = words.iter().filter(|w| *w != word).cloned().collect();
                if Solution::match_str(remaining, &words[..i].iter().chain(&words[i+1..]).cloned().collect()) {
                    return true;
                }
            }
        }
        false
    }

    pub fn find_substring1(s: String, words: Vec<String>) -> Vec<i32> {
        let mut ans = vec![];
        let s = s.as_bytes();

        let mut words_map: HM = HashMap::new();
        for w in words.iter().map(|w| w.as_bytes()) {
            *words_map.entry(w).or_insert(0) += 1;
        }

        let word_len = words[0].len();
        let all_words_len = words.len() * word_len;

        for offset in 0..word_len {
            let mut lack_map = words_map.clone();
            let mut begin = offset;
            let mut end = begin + all_words_len;
            let mut zeroed = 0;
            for word_begin in (begin..end - word_len).step_by(word_len) {
                if let Some(counter) = lack_map.get_mut(&s[word_begin..word_begin + word_len]) {
                    *counter -= 1;
                    if *counter == 0 {
                        zeroed += 1;
                    }
                }
            }
            while end <= s.len() {
                if let Some(counter) = lack_map.get_mut(&s[end - word_len..end]) {
                    *counter -= 1;
                    if *counter == 0 {
                        zeroed += 1;
                    }
                }
                if zeroed == lack_map.len() {
                    ans.push(begin as i32);
                }
                if let Some(counter) = lack_map.get_mut(&s[begin..begin + word_len]) {
                    *counter += 1;
                    if *counter == 1 {
                        zeroed -= 1;
                    }
                }
                begin += word_len;
                end += word_len;
            }
        }
        return ans;
    }


    // my design
    pub fn find_substring2(s: String, words: Vec<String>) -> Vec<i32> {
        let mut hash = HM::new();
        for w in words.iter().map(|s| s.as_bytes()) {
            *hash.entry(w).or_insert(0) += 1;
        }

        let s = s.as_bytes();
        let wlen = words[0].len();
        let word_count = words.len();
        for i in 0..wlen {
            let mut used_out = 0;
        }

        vec![]
    }
}


fn helper(strs: Vec<&str>) -> Vec<String> {
    let mut res = vec![];
    for s in strs {
        res.push(s.to_string());
    }
    res
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{helper, HM, Solution};

    #[test]
    fn test1() {
        let s = "barfoothefoobarman".to_string();
        let words = vec!["foo".to_string(), "bar".to_string()];
        let result = Solution::find_substring(s, words);

        // except [0, 9]
        println!("{result:?}");
    }

    #[test]
    fn test2() {
        let s = "wordgoodgoodgoodbestword".to_string();
        let words = vec!["word".to_string(), "good".to_string(), "best".to_string(), "word".to_string()];
        let result = Solution::find_substring(s, words);

        // except []
        println!("{result:?}");
    }

    #[test]
    fn test3() {
        let s = "barfoofoobarthefoobarman".to_string();
        let words = vec!["bar".to_string(), "foo".to_string(), "the".to_string()];
        let result = Solution::find_substring(s, words);

        // except [6, 9， 12]
        println!("{result:?}");
    }

    #[test]
    fn test4() {
        let s = "aaaaa".to_string();
        let words = vec!["aa".to_string(), "a".to_string(), "a".to_string()];
        let result = Solution::find_substring(s, words);

        // except [6, 9， 12]
        println!("{result:?}");
    }

    #[test]
    fn test5() {
        let s = "wordgoodgoodgoodbestword".to_string();
        let words = vec!["word".to_string(),"good".to_string(),"best".to_string(),"good".to_string()];
        let result = Solution::find_substring(s, words);

        println!("{result:?}");
    }

    #[test]
    fn test6() {
        let s = "barfoothefoobarman".to_string();
        let words = vec!["foo".to_string(), "bar".to_string()];
        let result = Solution::find_substring1(s, words);

        // except [0, 9]
        println!("{result:?}");
    }

    #[test]
    fn test7() { // time limit exceed if use my method
        let s = "abababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababab".to_string();
        let words = vec!["ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba"];
        let result = Solution::find_substring1(s, helper(words));

        println!("{result:?}");
    }

    #[test]
    fn test8() {
        let s = "oooooooooooooooooooooooooooooooofoobaroooooo".to_string();
        let words = vec!["foo".to_string(), "bar".to_string()];
        let result = Solution::find_substring1(s, words);

        // except [0, 9]
        println!("{result:?}");
    }

    #[test]
    fn test9() {
        let s = "foofoofoofoofoofoofoofoofoofoo".to_string();
        let words = vec!["foo".to_string(), "foo".to_string()];
        let result = Solution::find_substring1(s, words);

        // except [0, 9]
        println!("{result:?}");
    }

    #[test]
    fn test10() {
        let s = "foobarfoofoofoofoofoofoofoofoofoo".to_string();
        let words = vec!["foo".to_string(), "foo".to_string()];
        let result = Solution::find_substring1(s, words);

        // except [0, 9]
        println!("{result:?}");
    }

    #[test]
    fn test() {
        let words = vec!["bar".to_string(), "foo".to_string(), "the".to_string(), "inter".to_string()];
        let total = words.iter().fold(0, |x, s| x + s.len());
        println!("{}", total);

        let mut used = [false; 10];
        let len2 = 3;
        used[5] = true;
        if used[4..4+len2] != vec![false; len2] {
            println!("not all false");
        } else {
            println!("all false")
        }
        if used[6..6+len2] != vec![false; len2] {
            println!("not all false");
        } else {
            println!("all false")
        }


    }

    #[test]
    fn test_match_str1() {
        let s = "foobar";
        let v = vec!["foo".to_string(), "bar".to_string()];
        let k = Solution::match_str(s, &v);
        println!("{}", k);
    }

    #[test]
    fn test_match_str2() {
        let s = "aaaaa";
        let v = vec!["a".to_string(), "aaa".to_string(), "a".to_string()];
        let k = Solution::match_str(s, &v);
        println!("{}", k);
    }

    #[test]
    fn test_match_str3() {
        let s = "foobarworkwork";
        let v = vec!["foo".to_string(), "bar".to_string(), "work".to_string(), "work".to_string()];
        let k = Solution::match_str(s, &v);
        println!("{}", k);
    }

    #[test]
    fn test_match_str4() {
        let s = "foobarworkwork";
        let v = vec!["foo".to_string(), "bar".to_string(), "work".to_string(), "wor".to_string()];
        let k = Solution::match_str(s, &v);
        println!("{}", k);
    }

    #[test]
    fn test_match_str5() {
        let s = "foobarworkwork";
        let v = vec!["foo".to_string(), "bar".to_string(), "k".to_string(), "work".to_string(), "wor".to_string()];
        let k = Solution::match_str(s, &v);
        println!("{}", k);
    }

    #[test]
    fn test_match_str6() {
        let s = "refe";
        let v = vec!["e".to_string(), "ref".to_string()];
        let k = Solution::match_str(s, &v);
        println!("{}", k);
    }

    #[test]
    fn test_match_str7() {
        let s = "foobarworkwork";
        let v = vec!["foo".to_string(), "bar".to_string(), "k".to_string(), "work".to_string(), "wor".to_string()];
        let k = Solution::match_str(s, &v);
        println!("{}", k);
    }

    #[test]
    fn test_match_str8() {
        let s = "refe";
        let v = vec!["e".to_string(), "ref".to_string()];
        let k = Solution::match_str(s, &v);
        println!("{}", k);
    }

    #[test]
    fn test_chain() {
        let mut a = vec![1, 2, 3, 4, 5];
        let mut b = vec![6, 7, 8, 9, 10];
        let iter = a[..4].iter().chain(a[5..].iter()); // slice exceed index

        for i in iter {
            println!("{}", *i);
        }
    }

    #[test]
    fn test_hash_map() {
        let mut hash = HM::new();
        let words = helper(vec!["foo", "bar", "foo", "foo"]);

        for w in words.iter().map(|w| w.as_bytes()) {
            *hash.entry(w).or_insert(0) += 1;
            // if w is not exist, then insert 0
            // otherwise will get_mut
            // use the mut reference to self increment
        }

        for w in words.iter().map(|w| w.as_bytes()) {
            if let Some(value) = hash.get(w) {
                println!("{}", value);
            }
        }
    }

    #[test]
    fn test_trie_map() {
        let mut hash = HM::new();
        let words = helper(vec!["foo", "bar", "foo", "foo"]);

        for w in words.iter().map(|w| w.as_bytes()) {
            *hash.entry(w).or_insert(0) += 1;
            // if w is not exist, then insert 0
            // otherwise will get_mut
            // use the mut reference to self increment
        }

        for w in words.iter().map(|w| w.as_bytes()) {
            if let Some(value) = hash.get(w) {
                println!("{}", value);
            }
        }
    }
}