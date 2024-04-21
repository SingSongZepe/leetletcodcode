struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {

        false
    }
}



fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let string = String::from("ab");
        let p = String::from(".*");
        if Solution::is_match(string, p) {
            println!("matched");
        } else {
            println!("not matched");
        }
    }
}
