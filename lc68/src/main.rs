use std::cmp::max;

struct Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut result = vec![];
        let mut count = 0;
        let mut use_words = (0, 0);
        for (idx, word) in words.iter().enumerate() {
            if count + word.len() <= max_width as usize {
                count += word.len() + 1;
                use_words.1 = idx;
            } else {
                result.push(Solution::make_sentence(&words, use_words, max_width));
                count = word.len() + 1;
                use_words = (idx, idx);
            }
        }
        // last sentence
        result.push(Solution::make_sentence_last(&words, use_words, max_width));
        result
    }
    pub fn make_sentence(words: &Vec<String>, use_words: (usize, usize), max_width: i32) -> String {
        if use_words.1 == use_words.0 {
            return words[use_words.0].to_string() + &" ".repeat(max_width as usize - words[use_words.0].len());
        }
        let spaces = use_words.1 - use_words.0;
        let mut result = "".to_string();
        let total_space_length = max_width - (use_words.0..=use_words.1)
            .flat_map(|i| words[i].chars())
            .count() as i32;
        let mut count_of_extra_spaces = total_space_length % spaces as i32;
        for idx in use_words.0..use_words.1 {
            result.push_str(&words[idx]);
            result.push_str(&" ".repeat((total_space_length / spaces as i32) as usize));
            if count_of_extra_spaces > 0 {
                result.push_str(" ");
                count_of_extra_spaces -= 1;
            }
        }
        result.push_str(&words[use_words.1]); // last word
        result
    }
    pub fn make_sentence_last(words: &Vec<String>, use_words: (usize, usize), max_width: i32) -> String {
        // make words by index of use_words into a sentence, keep space == 1
        let mut result = "".to_string();
        for idx in use_words.0..=use_words.1 {
            result.push_str(&words[idx]);
            if idx < use_words.1 {
                result.push_str(" ");
            }
        }
        result.push_str(&" ".repeat(max_width as usize - result.len()));
        result
    }
}

// optimizing
impl Solution {
    pub fn make_sentence_last1(words: &Vec<String>, use_words: (usize, usize), max_width: i32) -> String {
        // (use_words.0..=use_words.1).fold("".to_string(), move |mut s, i| {
        //     s.push_str(words[i])
        // })
        (use_words.0..=use_words.1)
            .map(|idx| if idx < use_words.1 { words[idx].clone() + " " } else { words[idx].clone() })
            .collect::<String>() + &" ".repeat((max_width as usize).saturating_sub(
            (use_words.0..=use_words.1).fold(0, |acc, i| acc + words[i].len() + 1) - 1
        ))
    }
}

fn helper(words: Vec<&str>) -> Vec<String> {
    let mut results = vec![];
    for word in words {
        results.push(word.to_string());
    }
    results
}

fn print_text(text: &Vec<String>) {
    for line in text {
        println!("{}", line);
    }
    println!("");
}

#[cfg(test)]
mod tests {
    use crate::{
        Solution,
        helper,
        print_text,
    };

    #[test]
    fn test1() {
        let words = helper(vec!["This", "is", "an", "example", "of", "text", "justification."]);
        let max_width = 16;
        let result = Solution::full_justify(words, max_width);
        print_text(&result);
    }

    #[test]
    fn test2() {
        let words = helper(vec!["This", "is", "an", "example", "of", "text", "just.", "can"]);
        let max_width = 16;
        let result = Solution::full_justify(words, max_width);
        print_text(&result);
    }

    #[test]
    fn test3() {
        let words = helper(vec!["What","must","be","acknowledgment","shall","be"]);
        let max_width = 16;
        let result = Solution::full_justify(words, max_width);
        print_text(&result);
    }

    #[test]
    fn test4() {
        let words = helper(vec!["Science","is","what","we","understand","well","enough","to","explain","to","a","computer.","Art","is","everything","else","we","do"]);
        let max_width = 20;
        let result = Solution::full_justify(words, max_width);
        print_text(&result);
    }

    #[test]
    fn test_make_sentence() {
        let words = helper( vec!["This", "is", "an", "example", "of", "text", "justification."]);
        // let spaces = 3;
        let total_space_length = 8;
        let use_words = (0, 2);
        let result = Solution::make_sentence(&words, use_words, 16);
        println!("{}", result);
    }

    #[test]
    fn test_make_sentence_last1() {
        let words = helper(vec!["This", "is", "an", "example", "of", "text", "justification."]);
        let use_words = (6, 6);
        let max_width = 16;
        let result = Solution::make_sentence_last(&words, use_words, max_width);
        println!("{}", result);
    }

    #[test]
    fn test_make_sentence_last2() {
        let words = helper(vec!["What","must","be","acknowledgment","shall","be"]);
        let use_words = (4, 5);
        let max_width = 16;
        let result = Solution::make_sentence_last(&words, use_words, max_width);
        println!("{}", result);
    }

    #[test]
    fn test_make_sentence_last11() {
        let words = helper(vec!["This", "is", "an", "example", "of", "text", "justification."]);
        let use_words = (6, 6);
        let max_width = 16;
        let result = Solution::make_sentence_last1(&words, use_words, max_width);
        println!("{}", result);
    }

    #[test]
    fn test_make_sentence_last21() {
        let words = helper(vec!["What","must","be","acknowledgment","shall","be"]);
        let use_words = (4, 5);
        let max_width = 16;
        let result = Solution::make_sentence_last1(&words, use_words, max_width);
        println!("{}", result);
    }
}

fn main() {
    println!("Hello, world!");
}
