
struct Solution;

impl Solution {
    pub fn multiply1(num1: String, num2: String) -> String {
        let a = Number::from_string(num1);
        let b = Number::from_string(num2);
        a.multiply(b).to_string()
    }
}

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let v1: Vec<char> = num1.chars().rev().collect();
        let v2: Vec<char> = num2.chars().rev().collect();
        let mut carry = 0;
        let mut res: Vec<u8> = vec![0; v1.len() + v2.len()];

        for i in 0..v1.len() {
            for j in 0..v2.len() {
                let t = v1[i].to_digit(10).unwrap() * v2[j].to_digit(10).unwrap() + carry as u32;
                carry = t / 10 ;
                let tmp = (t % 10) as u8;
                Solution::push(&mut res, i + j, tmp);
            }
            if carry > 0 {
                Solution::push(&mut res, i + v2.len(), carry as u8);
                carry = 0;
            }
        }

        Solution::vec_u8_to_string(res)
    }

    // we dont need to push every time when we add res[i+j]
    pub fn push(res: &mut Vec<u8>, mut from: usize, m: u8) {
        res[from] += m;
        while res[from] > 9 {
            res[from + 1] += 1;
            res[from] = res[from] % 10;
            from += 1;
        }
    }

    fn vec_u8_to_string(vec: Vec<u8>) -> String {
        if vec.iter().all(|x| *x == 0) {
            return "0".to_string();
        }
        vec.iter()
            .rev()
            .skip_while(|&&x| x == 0)
            .map(|&x| (x + 48) as char)
            .collect()
    }
}

struct Number {
    digits: Vec<u8>,
}

impl Number {
    fn from_string(s: String) -> Self {
        Number {
            digits: s.chars().rev().map(|c| ((c as u32) - '0' as u32) as u8).collect(),
        }
    }

    fn multiply(self, other: Number) -> Number {
        let mut result = vec![0; self.digits.len() + other.digits.len()];
        self.digits.iter().enumerate().flat_map(|(i, a)| {
            other.digits.iter().enumerate().map(move |(j, &b)| (i + j, a * b))
        })
        .for_each(|(indx, product)| {
            Self::helper(&mut result, indx, product % 10);
            Self::helper(&mut result, indx + 1, product / 10);
        });
        // pop zero
        while result.len() > 1 && result.last() == Some(&0) {
            result.pop();
        }
        Number { digits: result }
    }

    fn to_string(self) -> String {
        self.digits.iter().rev().map(|&d| char::from_u32(d as u32 + '0' as u32).unwrap()).collect()
    }

    #[inline]
    fn helper(result: &mut Vec<u8>, mut k: usize, val: u8) {
        result[k] += val;
        while result[k] >= 10 {
            result[k] -= 10;
            k += 1;
            result[k] += 1;
        }
    }

    // insight
    fn see_iter(self, other: Number) {
        let mut result = vec![0; self.digits.len() + other.digits.len()];
        let iter = self.digits.iter().enumerate().flat_map(|(i, a)| {
            other.digits.iter().enumerate().map(move |(j, &b)| (i + j, a * b))
        });
        for (i, u) in iter {
            println!("{} {}", i, u);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Number, Solution};

    #[test]
    fn test1() {
        let num1 = "2".to_string();
        let num2 = "3".to_string();
        let result = Solution::multiply(num1, num2);
        println!("{}", result);
    }

    #[test]
    fn test2() {
        let num1 = "123".to_string();
        let num2 = "456".to_string();
        let result = Solution::multiply(num1, num2);
        println!("{}", result);
    }

    #[test]
    fn test4() {
        let num1 = "0".to_string();
        let num2 = "0".to_string();
        let result = Solution::multiply(num1, num2);
        println!("{}", result);
    }

    #[test]
    fn test5() {
        let num1 = "9".to_string();
        let num2 = "9".to_string();
        let result = Solution::multiply(num1, num2);
        println!("{}", result);
    }

    #[test]
    fn test6() {
        let num1 = "312412314121512312512321523523532513515135135432".to_string();
        let num2 = "574736546466527663745646724356725437624542575653423435".to_string();
        let result = Solution::multiply(num1, num2);
        println!("{}", result);
    }

    #[test]
    fn test21() {
        let num1 = Number::from_string("123".to_string());
        let num2 = Number::from_string("456".to_string());
        let result = num1.multiply(num2);
        println!("{}", result.to_string());
    }

    #[test]
    fn test41() {
        let num1 = Number::from_string("0".to_string());
        let num2 = Number::from_string("0".to_string());
        let result = num1.multiply(num2);
        println!("{}", result.to_string());
    }

    #[test]
    fn test51() {
        let num1 = Number::from_string("9".to_string());
        let num2 = Number::from_string("9".to_string());
        let result = num1.multiply(num2);
        println!("{}", result.to_string());
    }

    #[test]
    fn test61() {
        let num1 = Number::from_string("312412314121512312512321523523532513515135135432".to_string());
        let num2 = Number::from_string("574736546466527663745646724356725437624542575653423435".to_string());
        let result = num1.multiply(num2);
        println!("{}", result.to_string());
    }

    // insight
    #[test]
    fn test_see_iter1() {
        let num1 = Number::from_string("26".to_string());
        let num2 = Number::from_string("99".to_string());
        num1.see_iter(num2);
    }
    #[test]
    fn test_see_iter2() {
        let num1 = Number::from_string("312412314121512312512321523523532513515135135432".to_string());
        let num2 = Number::from_string("574736546466527663745646724356725437624542575653423435".to_string());
        num1.see_iter(num2);
    }

    #[test]
    fn test_push() {
        let mut v = Vec::<u8>::from([9, 9, 9, 9, 0]);
        let r = 7;
        let result = Solution::push(&mut v, 0, r);
        println!("{v:?}");
    }

    #[test]
    fn test_with_capacity() {
        // let mut v = Vec::<u8>::with_capacity(10);
        let mut v = vec![0; 10];
        v[8] = 1;
        println!("{v:?}");
    }
}



fn main() {
    println!("Hello, world!");
}
