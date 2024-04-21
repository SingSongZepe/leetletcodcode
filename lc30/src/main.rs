struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {

        vec![]
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
    fn test1() {
        let s = "barfoofoobarthefoobarman".to_string();
        let words = vec!["bar".to_string(), "foo".to_string(), "the".to_string()];
        let result = Solution::find_substring(s, words);

        // except [6, 9， 12]
        println!("{result:?}");
    }
}