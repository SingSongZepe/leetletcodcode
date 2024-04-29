use std::fmt::format;

struct Solution;

impl Solution {

    // 5% at speed, it's too low
    // 95% at memory, not bad
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return String::from("1");
        }

        let mut s = String::default();
        let str = Solution::count_and_say(n-1);

        let mut i = 0;
        while i < str.len() {
            if let Some(c) = str.chars().nth(i) {
                let left = i;
                let right = str.char_indices().skip(i+1).find(|(_, x)| *x != str.chars().nth(i).unwrap()).map(|(index, _)| index - 1).unwrap_or(str.len() - 1);
                s.push_str(format!("{}", right-left+1).as_str());
                s.push(c);
                i = right + 1;
            } else {
                break;
            }
        }

        s
    }

    pub fn count_and_say1(n: i32) -> String {
        if n == 1 {
            return "1".to_string();
        }

        let mut output: Vec<char> = vec![];

        let mut last_char = ' ';
        // Using the ascii position for the counts to make conversion to string easier later
        let mut count: u8 = '0' as u8;
        for char in Solution::count_and_say(n - 1).chars().into_iter() {
            if last_char == ' ' {
                last_char = char;
                count = '1' as u8;
            } else if char == last_char {
                count += 1;
            } else {
                output.push(count as char);
                output.push(last_char);
                last_char = char;
                count = '1' as u8;
            }
        }
        output.push(count as char);
        output.push(last_char);
        output.into_iter().collect()
    }

}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let n = 1;
        let result = Solution::count_and_say(n);
        println!("{result}");
    }

    #[test]
    fn test2() {
        let n = 2;
        let result = Solution::count_and_say(n);
        println!("{result}");
    }

    #[test]
    fn test3() {
        let n = 3;
        let result = Solution::count_and_say(n);
        println!("{result}");
    }

    #[test]
    fn test4() {
        let n = 4;
        let result = Solution::count_and_say(n);
        println!("{result}");
    }

    #[test]
    fn test5() {
        let n = 30;
        let result = Solution::count_and_say(n);
        println!("{result}");
    }

    #[test]
    fn test_find() {
        fn find_left_right_indices(s: &str) {
            let mut chars = s.chars().enumerate().peekable();

            while let Some((left, c)) = chars.next() {
                let right = chars.peek().map_or(s.len() - 1, |(right, x)| if *x != c { *right - 1 } else { *right });

                println!("Character: {}, Left Index: {}, Right Index: {}", c, left, right);
            }
        }

        let input = "11";
        find_left_right_indices(input);
    }

    #[test]
    fn test_iter() {
        for mut i in 0..10 {
            i += 1;
            println!("{}", i);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
