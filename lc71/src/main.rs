
struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack : Vec<String> = vec![];
        for dir in path.split("/").filter(|s| !s.is_empty()) {
            match dir {
                "." => {
                    continue;
                },
                ".." => {
                    if !stack.is_empty() {
                        stack.pop();
                    }
                },
                _ => {
                    stack.push(dir.to_string());
                }
            }
        }
        let a = stack.iter().fold(String::new(), |mut acc, s| {
            acc.push('/');
            acc.push_str(&s);
            acc
        });
        if a.is_empty() {
            "/".to_string()
        } else {
            a
        }
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

    #[test]
    fn test2() {
        let path = "/home//foo".to_string();
        let result = Solution::simplify_path(path);
        println!("{}", result);
    }

    #[test]
    fn test3() {
        let path = "/home/user/Documents/../Pictures".to_string();
        let result = Solution::simplify_path(path);
        println!("{}", result);
    }

    #[test]
    fn test4() {
        let path = "/../".to_string();
        let result = Solution::simplify_path(path);
        println!("{}", result);
    }

    #[test]
    fn test5() {
        let path = "/.../a/../b/c/../d/./".to_string();
        let result = Solution::simplify_path(path);
        println!("{}", result);
    }

    #[test]
    fn test_macro() {
        let v = vec![1, 2, 3, 4];
    }

    #[test]
    fn test_split() {
        let path = "/home//foo".to_string();
        for dir in path.split("/").filter(|s| !s.is_empty()) {
            println!("{}", dir);
        }
    }

    #[test]
    fn test_stack() {
        let stack: Vec<String> = vec!["a".to_string(), "b".to_string()];
        let result = stack.iter().fold(String::new(), |mut acc, s| {
            acc.push('/');
            acc.push_str(&s);
            acc
        });
        println!("{}", result);
    }
}

// macro_rules! mvec {
//     () => {
//         $crate::__rust_force_expr!($crate::vec::Vec::new())
//     }
// }

fn main() {
    println!("Hello, world!");
}
