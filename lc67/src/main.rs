use std::cmp::min;

struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut carry = false;
        let mut p1: Vec<char>;
        let mut p2: Vec<char>;
        if a.len() < b.len() {
            p2 = a.chars().rev().into_iter().collect::<Vec<char>>();
            p1 = b.chars().rev().into_iter().collect::<Vec<char>>();
        } else {
            p1 = a.chars().rev().into_iter().collect::<Vec<char>>();
            p2 = b.chars().rev().into_iter().collect::<Vec<char>>();
        }
        for i in 0..p2.len() {
            match (p1[i], p2[i]) {
                ('0', '0') => {
                    if carry {
                        p1[i] = '1';
                        carry = false;
                    }
                },
                ('0', '1') | ('1', '0') => {
                    if !carry {
                        p1[i] = '1';
                    } else {
                        p1[i] = '0';
                        carry = true;
                    }
                },
                ('1', '1') => {
                    if carry {
                       p1[i] = '1';
                    } else {
                        p1[i] = '0';
                    }
                    carry = true;
                },
                _ => {}
            }
        }
        for i in p2.len()..p1.len() {
            if carry {
                match p1[i] {
                    '0' => {
                        p1[i] = '1';
                        carry = false;
                        break;
                    },
                    '1' => {
                        p1[i] = '0';
                    },
                    _ => {}
                }
            }
        }
        // let l1 = p1.len();
        if carry {
           p1.push('1');
        }
        p1.reverse();
        p1.iter().collect::<String>()
    }
}

struct Solution1;

impl Solution1 {
    pub fn add_binary(a: String, b: String) -> String {
        // Swap pointers so a is largest
        let mut a = &a;
        let mut b = &b;
        if a.len()<b.len() {
            std::mem::swap(&mut a, &mut b);
        }

        // unsafe from_utf8_unchecked is compiled to no-op,
        // otherwise the string is rechecked as valid utf8
        let mut res = a
            .as_bytes().rchunks(127).zip(
            b.as_bytes().rchunks(127).chain(std::iter::repeat(&[b'0'; 127][..])),
        )
            .fold((String::new(), 0), |s, (a, b)| {
                let sum = unsafe {
                    s.1 + u128::from_str_radix(std::str::from_utf8_unchecked(a), 2).unwrap_or(0)
                        + u128::from_str_radix(std::str::from_utf8_unchecked(b), 2).unwrap_or(0)
                };
                (format!("{:0127b}", sum & 0x7fffffffffffffffffffffffffffffff) + &s.0, sum >> 127)
            });

        // final carry and special cases
        if (res.1 == 1) {
            "1".to_string() + &res.0
        } else {
            let str = res.0.trim_start_matches("0").to_string();
            if str.len() > 0 {
                str
            } else {
                "0".to_string()
            }
        }
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
        let a = "11".to_string();
        let b = "1".to_string();
        let result = Solution::add_binary(a, b);
        println!("{result}");
    }

    #[test]
    fn test2() {
        let a = "1010".to_string();
        let b = "1011".to_string();
        let result = Solution::add_binary(a, b);
        println!("{result}");
    }

    #[test]
    fn test3() {
        let a = "101010111011".to_string();
        let b = "11010101100101".to_string();
        let result = Solution::add_binary(a, b);
        println!("{result}");
    }

    #[test]
    fn test4() {
        let a = "101111".to_string();
        let b = "10".to_string();
        let result = Solution::add_binary(a, b);
        println!("{result}"); // excepted 110001
    }

    #[test]
    fn test11() {
        let a = "11".to_string();
        let b = "1".to_string();
        let result = Solution1::add_binary(a, b);
        println!("{result}");
    }

    #[test]
    fn test21() {
        let a = "1010".to_string();
        let b = "1011".to_string();
        let result = Solution1::add_binary(a, b);
        println!("{result}");
    }

    #[test]
    fn test31() {
        let a = "101010111011".to_string();
        let b = "11010101100101".to_string();
        let result = Solution1::add_binary(a, b);
        println!("{result}");
    }

    #[test]
    fn test41() {
        let a = "101111".to_string();
        let b = "10".to_string();
        let result = Solution1::add_binary(a, b);
        println!("{result}"); // excepted 110001
    }

    #[test]
    fn test_rchunks() {
        let a = "131294129409712098309128412".to_string();
        // a.as_bytes().rchunks(5).for_each(|chunk| {
        //     println!("{:?}", std::str::from_utf8(chunk).unwrap());
        // });
        let b = "dlwiajkhkawkhekwahkjfhhwadwa".to_string();
        // b.as_bytes().rchunks(5).for_each(|chunk| {
        //     println!("{:?}", std::str::from_utf8(chunk).unwrap());
        // });

        let s = a
            .as_bytes().rchunks(5).zip(
                b
                    .as_bytes()
                    .rchunks(5)
                    .chain(std::iter::repeat(&[b'0'; 5][..])),
            )
            .fold((String::new(), 0), |s, (a, b)| {
                println!("a: {:?}, b: {:?}", std::str::from_utf8(a).unwrap(), std::str::from_utf8(b).unwrap());
                (s.0, 0)
            });
    }
}

fn main() {
    println!("Hello, world!");
}
