struct  Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut left: usize = 0;
        let mut been: [i32; 256] = [-1; 256];
        let mut max: i32  = 0;

        for (idx, c) in s.char_indices() {
            let ch = c as usize;
            if been[ch] >= 0 {
                if been[ch] >= left as i32 {
                    left = been[ch] as usize + 1;
                }
            }

            max = if max > (idx - left) as i32 {
                max
            } else {
                (idx - left) as i32 + 1
            };
            been[ch] = idx as i32;
        }

        max
    }
}

fn main() {
    let text = "abba";
    println!("{}", Solution::length_of_longest_substring(text.to_string()));
}
