use std::ops::Index;

struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        // let num_rows = num_rows - 1;
        if num_rows == 1 {
            return  s;
        }
        let num_rows = num_rows - 1;
        let mut rs = String::from("");
        let mut sub = 0;
        let len = s.len();

        while (sub <= num_rows) {
            if sub == 0 || sub == num_rows {
                let mut i = 0;
                while (num_rows * i + sub < len as i32) {
                    if let Some(ch) = s.chars().nth((num_rows * i + sub) as usize) {
                        rs.push(ch);
                    }
                    i += 2;
                }
            } else { // sub 1 2 3
                let mut i = 0;
                loop {
                    if num_rows * i + sub > len as i32 {
                        break;
                    }
                    if let Some(ch) = s.chars().nth((num_rows * i + sub) as usize) {
                        rs.push(ch);
                    }
                    if let Some(ch) = s.chars().nth((num_rows * (i + 2) - sub) as usize) {
                        rs.push(ch);
                    }
                    i += 2;
                }
            }
            sub += 1;
        }

        rs
    }
    pub fn convert1(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let num_rows = num_rows - 1;
        let mut rs = String::from("");
        let len = s.len() as i32;

        for i in 0..=num_rows {
            let mut j = i;
            let mut flag = true;
            while j < len {
                rs.push(s.chars().nth(j as usize).unwrap());
                if i != 0 && i != num_rows {
                    let next = j + 2 * (num_rows - i);
                    if flag && next < len {
                        rs.push(s.chars().nth(next as usize).unwrap());
                    }
                }
                j += 2 * num_rows;
                flag = !flag;
            }
        }

        rs
    }
}

fn main() {
    println!("{}", Solution::convert1(String::from("ABC"), 2));
}
