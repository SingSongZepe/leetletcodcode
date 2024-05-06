
struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {

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
        let path = "/home/".to_string();
        let result = Solution::simplify_path(path);
        println!("{}", result);
    }
}

fn main() {
    println!("Hello, world!");
}
