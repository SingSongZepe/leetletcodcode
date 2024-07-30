
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
    fn last(&self) -> Option<&T> {
        self.data.last()
    }
}

struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = Stack::new();
        let mut pos = vec![false; s.len()];

        for (idx, ch) in s.chars().into_iter().enumerate() {
            match ch {
                '(' => {
                    stack.push((idx, ch));
                }
                ')' => {
                    if let Some(c) = stack.last() {
                        if c.1 == '(' {
                            if let Some(v) = stack.pop() {
                                pos[v.0] = true;
                                pos[idx] = true;
                            }
                        } else {
                            stack.push((idx, ch));
                        }
                    }
                }
                _ => {
                    panic!("unknown letter occurred");
                }
            }
        }

        // let mut max = 0;
        // let mut count = 0;
        // for b in pos {
        //     if b {
        //         count += 1;
        //     } else {
        //         max = max.max(count);
        //         count = 0;
        //     }
        // }
        // max = max.max(count);

        let mut max = 0;
        let mut count = 0;
        for b in pos {
            count = if b {
                count + 1
            } else {
                0
            };
            max = max.max(count);
        }

        max
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
        let s = "(()".to_string();
        let res = Solution::longest_valid_parentheses(s);
        println!("{}", res);
    }

    #[test]
    fn test2() {
        let s = ")()())".to_string();
        let res = Solution::longest_valid_parentheses(s);
        println!("{}", res);
    }

    #[test]
    fn test3() {
        let s = "".to_string();
        let res = Solution::longest_valid_parentheses(s);
        println!("{}", res);
    }

    #[test]
    fn test4() {
        let s = "(())".to_string();
        let res = Solution::longest_valid_parentheses(s);
        println!("{}", res);
    }

    #[test]
    fn test5() {
        let s = "((())(".to_string();
        let res = Solution::longest_valid_parentheses(s);
        println!("{}", res);
    }


    #[test]
    fn test6() {
        let s = "()(()".to_string();
        let res = Solution::longest_valid_parentheses(s);
        println!("{}", res);
    }

    #[test]
    fn test7() {
        let s = "()((())(()()()".to_string();
        let res = Solution::longest_valid_parentheses(s);
        println!("{}", res);
    }
}


