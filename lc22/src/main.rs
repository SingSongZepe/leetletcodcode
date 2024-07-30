use std::fmt::format;

struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        // suppose n >= 1
        if n == 1 {
            vec!["()".to_string()]
        } else {
            Solution::g("(".to_string(),n-1, n)
        }
    }
    pub fn g(from: String, lr: i32, rr: i32) -> Vec<String> {
        if lr < 0 || rr < 0 || lr > rr {
            return vec![];
        }
        if lr == 0 && rr == 0 {
            return vec![from];
        }
        let mut fl = from.to_owned();
        fl.push_str("(");
        let mut vl = Solution::g(fl,lr - 1, rr);

        let mut fr = from.to_owned();
        fr.push_str(")");
        let vr = Solution::g(fr, lr, rr - 1);

        vl.extend(vr);

        vl
    }

    pub fn generate_parenthesis1(n: i32) -> Vec<String> {
        if n == 1 {
            vec!["()".to_string()]
        } else {
            Self::generate("(".to_owned(), n-1, n)
        }
    }
    fn generate(builder: String, openers: i32, closers: i32) -> Vec<String> {
        if openers < 0 || closers < 0 || closers < openers {
            return vec![];
        }
        if openers == 0 && closers == 0 {
            return vec![builder];
        }

        let mut builder1 = builder.to_owned();
        builder1.push_str("(");
        let mut par1 = Self::generate(builder1,openers - 1, closers);

        let mut builder2 = builder.to_owned();
        builder2.push_str(")");
        let par2 = Self::generate(builder2, openers, closers - 1);

        par1.extend(par2);
        par1
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
        let n = 1;
        let res = Solution::generate_parenthesis(n);
        println!("t1 {res:?}");
    }

    #[test]
    fn test2() {
        let n = 2;
        let res = Solution::generate_parenthesis(n);
        println!("t2 {res:?}");
    }

    #[test]
    fn test3() {
        let n = 3;
        let res = Solution::generate_parenthesis(n);
        println!("t3 {res:?}");
    }
}