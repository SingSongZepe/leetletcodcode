
struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {


        false
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let s = "aa".to_string();
        let p = "a".to_string();
        let result = Solution::is_match(s, p);
        println!("{result}");
    }
}

fn main() {
    println!("Hello, world!");
}
