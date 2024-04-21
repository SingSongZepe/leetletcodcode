struct  Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len();

        let mut rl : usize = 0;
        let mut rr : usize = 0;

        for i in 0..len {
            // odd
            let mut r: i32 = i as i32 + 1;
            let mut l: i32 = i as i32 - 1;
            while (l >= 0 && r <= len as i32 && s.chars().nth(r as usize) == s.chars().nth(l as usize)) {
                l -= 1;
                r += 1;
            }
            if r - l - 2 > rr  as i32  - rl as i32 {
                rr = (r - 1) as usize;
                rl = (l + 1) as usize;
            }

            r = i as i32 + 1;
            l = i as i32;
            while (l >= 0 && r <= len as i32 && s.chars().nth(r as usize) == s.chars().nth(l as usize)) {
                l -= 1;
                r += 1;
            }
            if r - l - 2 > rr  as i32  - rl as i32 {
                rr = (r - 1) as usize;
                rl = (l + 1) as usize;
            }
        }

        String::from(&s[rl..rr+1])
    }

    // fn expand(s: &[u8], i: usize) -> (usize, usize) {
    //     let n: i32 = s.len() as i32;
    //
    //     let (mut i1, mut j1, mut d1): (i32, i32, i32) = (i as i32, (i + 1) as i32, 0 as i32);
    //     while i1 >= 0  && j1 < n && s[i1 as usize] == s[j1 as usize] {
    //         i1 -= 1;
    //         j1 += 1;
    //         d1 += 2;
    //     }
    //
    //     let (mut i2, mut j2, mut d2): (i32, i32, i32) = ((i - 1) as i32, (i + 1) as i32, 0 as i32);
    //     while i2 >= 0 && j2 < n && s[i2 as usize] == s[j2 as usize] {
    //         i2 -= 1;
    //         j2 += 1;
    //         d2 += 2;
    //     }
    //
    //     if d1 > d2 {
    //         (d1 as usize, (j1 - d1) as usize)
    //     } else {
    //         (d2 as usize, (j2 - d2) as usize)
    //     }
    // }
    //
    // pub fn longest_palindrome1(s: String) -> String {
    //     let str = s.as_bytes();
    //     let (mut d_res, mut p_res) = (0, 0);
    //
    //     for i in 0 .. s.len() {
    //         let (d, p) = Self::expand(str, i);
    //         if d > d_res {
    //             d_res = d;
    //             p_res = p;
    //         }
    //     }
    //
    //     String::from(&s[d_res..p_res])
    // }
}

fn main() {
    // println!("Hello, world!");
    println!("{}", Solution::longest_palindrome1(String::from("aaaa")));
}


