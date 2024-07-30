use std::sync::mpsc::channel;

struct Solution;

struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack { data: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
    }
    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
    fn is_empty(&self) -> bool {
        return self.data.is_empty();
    }
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 != 0 {
            return false;
        }

        let mut stack: Stack<char>  = Stack::new();
        let mut iter = s.chars();

        while let Some(c) = iter.next() {
            match c {
                ')' => {
                    if stack.is_empty() {
                        return false;
                    }
                    if let Some(ch) = stack.pop() {
                        if ch != '(' {
                            return false;
                        }
                    }
                }
                ']' => {
                    if stack.is_empty() {
                        return false;
                    }
                    if let Some(ch) = stack.pop() {
                        if ch != '[' {
                            return false;
                        }
                    }
                }
                '}' => {
                    if stack.is_empty() {
                        return false;
                    }
                    if let Some(ch) = stack.pop() {
                        if ch != '{' {
                            return false;
                        }
                    }
                }
                _ => {
                    stack.push(c);
                }
            }
        }

        if stack.is_empty() {
            true
        } else {
            false
        }
    }

    pub fn is_valid1(s: String) -> bool {
        if s.len() % 2 != 0 {
            return false;
        }

        let mut stack: Stack<char> = Stack::new();
        let chars = s.chars();

        for ch in chars {
            if stack.is_empty() {
                stack.push(ch);
                continue;
            }
            match (ch) {
                ')' => {
                    if let Some(c) = stack.pop() {
                        if c != '(' {
                            return false;
                        }
                    }
                }
                ']' => {
                    if let Some(c) = stack.pop() {
                        if c != '[' {
                            return false;
                        }
                    }
                }
                '}' => {
                    if let Some(c) = stack.pop() {
                        if c != '{' {
                            return false;
                        }
                    }
                }
                _ => {
                    stack.push(ch);
                }
            }
        }

        if stack.is_empty() {
            true
        } else {
            false
        }
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
        let s = "()[]{}".to_string();
        let result = Solution::is_valid(s);
        println!("t1 {}", result);
    }

    #[test]
    fn test2() {
        let s = "(]".to_string();
        let result = Solution::is_valid(s);
        println!("t2 {}", result);
    }

    #[test]
    fn test3() {
        let s = "]".to_string();
        let result = Solution::is_valid(s);
        println!("t3 {}", result);
    }

    #[test]
    fn test4() {
        let s = "[[}}]".to_string();
        let result = Solution::is_valid(s);
        println!("t4 {}", result);
    }

    #[test]
    fn test5() {
        let s = "[[]][]".to_string();
        let result = Solution::is_valid1(s);
        println!("t5 {}", result);
    }

    // #[test]
    // fn test6() {
    //     let s = "[[]][]".to_string();
    //     let result = Solution::is_valid2(s);
    //     println!("t6 {}", result);
    // }
}