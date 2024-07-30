

struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let mut minmap = vec![vec![-1; word2.len()]; word1.len()];
        Solution::get_cost(word1, word2, 0, 0, &mut minmap)
    }
    pub fn get_cost(word1: &[u8], word2: &[u8], from1: usize, from2: usize, minmap: &mut Vec<Vec<i32>>) -> i32 {
        if from1 == word1.len() {
            return (word2.len() - from2) as i32; // insert remaining chars
        } else if from2 == word2.len() {
            return (word1.len() - from1) as i32; // delete remaining chars
        }
        if minmap[from1][from2] != -1 {
            return minmap[from1][from2];
        }
        // same
        if word1[from1] == word2[from2] {
            return Solution::get_cost(word1, word2, from1 + 1, from2 + 1, minmap);
        }
        let insert = Solution::get_cost(word1, word2, from1, from2 + 1, minmap) + 1;
        let delete = Solution::get_cost(word1, word2, from1 + 1, from2, minmap) + 1;
        let replace = Solution::get_cost(word1, word2, from1 + 1, from2 + 1, minmap) + 1;
        let answer = insert.min(delete).min(replace);
        minmap[from1][from2] = answer;
        answer
    }
}

struct Solution1;

impl Solution1 {
    fn get_cost(word1: &[u8], word2: &[u8], dp: &mut Vec<Vec<u16>>) -> u16 {
        if word2.is_empty() {
            return word1.len() as u16; // delete remaining chars
        }
        if word1.is_empty() {
            return word2.len() as u16; // insert remaining chars
        }
        // let answer = dp[word1.len() - 1][word2.len() - 1];
        if dp[word1.len() - 1][word2.len() - 1] != u16::MAX {
            return dp[word1.len() - 1][word2.len() - 1];
        }

        if word1[0] == word2[0] {
            return Self::get_cost(&word1[1..], &word2[1..], dp); // use that letter
        }

        // insert or replace or delete
        let insert_cost = Self::get_cost(&word1[..], &word2[1..], dp);
        let replace_cost = Self::get_cost(&word1[1..], &word2[1..], dp);
        let delete_cost = Self::get_cost(&word1[1..], &word2[..], dp);

        let answer = insert_cost.min(replace_cost).min(delete_cost) + 1;
        dp[word1.len() - 1][word2.len() - 1] = answer;
        answer
    }
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let mut dp = vec![vec![u16::MAX; word2.len()]; word1.len()];
        Self::get_cost(&word1, &word2, &mut dp) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Solution,
        Solution1,
    };


    #[test]
    fn test1() {
        let word1 = "horse".to_string();
        let word2 = "ros".to_string();
        let result = Solution::min_distance(word1, word2);
        println!("{}", result);
    }

    #[test]
    fn test2() {
        let word1 = "intention".to_string();
        let word2 = "execution".to_string();
        let result = Solution::min_distance(word1, word2);
        println!("{}", result);
    }

    #[test]
    fn test11() {
        let word1 = "horse".to_string();
        let word2 = "ros".to_string();
        let result = Solution1::min_distance(word1, word2);
        println!("{}", result);
    }

    #[test]
    fn test21() {
        let word1 = "intention".to_string();
        let word2 = "execution".to_string();
        let result = Solution1::min_distance(word1, word2);
        println!("{}", result);
    }

    #[test]
    fn test_u16() {
        let a = u16::MAX;
        println!("{}", a as i32 + 1 == 0);
    }
}

fn main() {
    println!("Hello, world!");
}
