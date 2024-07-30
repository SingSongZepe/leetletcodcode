
struct Solution;

impl Solution {
    // beats 100% at speed, but 35% at memory
    pub fn letter_combinations(mut digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let dl: Vec<&str> = vec![
            "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
        ];
        let mut res : Vec<String> = vec![];
        for ch in digits.chars() {
            if res.is_empty() {
                res = dl[ch as usize - 50].chars().map(|c| c.to_string()).collect();
                continue;
            } else {
                res = res.iter().flat_map(|s| {
                    dl[ch as usize - 50].chars().map(move |c| {
                        format!("{}{}", s, c)
                    })
                }).collect();
            }
        }
        res
    }

    // beats 100% at speed, 93% at memory, not bad
    pub fn letter_combinations1(mut digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let dl: Vec<&str> = vec![
            "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
        ];
        let mut res : Vec<String> = vec!["".to_string()]; // 初始化一个包含空字符串的向量
        for ch in digits.chars() {
            res = res.iter().flat_map(|s| {
                dl[ch as usize - 50].chars().map(move |c| format!("{}{}", s, c))
            }).collect();
        }
        res
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
        let digits = String::from("234");
        let result = Solution::letter_combinations(digits);
        println!("t1 {result:?}");
    }

    #[test]
    fn test2() {
        let digits = String::from("");
        let result = Solution::letter_combinations(digits);
        println!("t2 {result:?}");
    }

    #[test]
    fn test3() {
        let digits = String::from("2");
        let result = Solution::letter_combinations(digits);
        println!("t3 {result:?}");
    }

    #[test]
    fn test4() {
        let digits = String::from("234");
        let result = Solution::letter_combinations1(digits);
        println!("t4 {result:?}");
    }

    #[test]
    fn test5() {
        let digits = String::from("");
        let result = Solution::letter_combinations1(digits);
        println!("t5 {result:?}");
    }

    #[test]
    fn test6() {
        let digits = String::from("2");
        let result = Solution::letter_combinations1(digits);
        println!("t6 {result:?}");
    }

    // #[test]
    // fn test() {
    //     // let mut v = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    //     let mut v_mux = vec!["d", "e", "f"];
    //     let v: Vec<String> = vec![];
    //
    //     let new_v: Vec<String> = v.iter().flat_map(|s| {
    //         v_mux.iter().map(move |c| {
    //             format!("{}{}", s, c)
    //         })
    //     }).collect();
    //
    //     // println!("{}", v_mux[0]);
    //
    //     println!("{new_v:?}");
    // }
}
