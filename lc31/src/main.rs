
struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let len = nums.len();
        let mut idx = len;
        for i in (1..len).rev() {
            if nums[i] > nums[i-1] {
                idx = i;
                break;
            }
        }
        if idx == len {
            nums.reverse();
            return;
        }
        let l = nums[idx-1];
        let mut nli = len - 1;
        for (i, &n) in nums.iter().enumerate().rev() {
            if n > l {
                nli = i;
                break;
            }
        }
        nums.swap(idx-1, nli);
        nums[idx..].reverse()
    }

    pub fn next_permutation1(nums: &mut Vec<i32>) {
        let len = nums.len();
        let mut idx = len;
        for i in (0..(len-1)).rev(){
            if nums[i] < nums[i+1] { idx = i; break; }
        }
        if idx == len { nums.reverse(); return; }
        let l = nums[idx];
        let mut nli = len - 1;
        for (i, &n) in nums.iter().enumerate().rev(){
            if n > l { nli = i; break; }
        }
        nums.swap(idx, nli);
        nums[(idx+1)..].reverse();
    }

    pub fn next_permutation2(nums: &mut Vec<i32>) {
        for (l, r) in (1..nums.len()).zip(0..(nums.len() - 1)).rev() {
            if nums[l] > nums[r] {
                let (left, right) = nums.split_at_mut(l);
                std::mem::swap(
                    right
                        .iter_mut()
                        .min_by_key(|key| match **key > left[r] {
                            true => **key,
                            false => i32::MAX,
                        })
                        .unwrap(),
                    &mut left[r],
                );
                right.sort();
                return;
            }
        }
        nums.reverse();
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
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        println!("{nums:?}");
    }

    #[test]
    fn test2() {
        let mut nums = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        println!("{nums:?}");
    }

    #[test]
    fn test3() {
        let mut nums = vec![1, 1, 5];
        Solution::next_permutation(&mut nums);
        println!("{nums:?}");
    }

    #[test]
    fn test4() {
        let mut nums = vec![1, 1, 5, 4];
        Solution::next_permutation(&mut nums);
        println!("{nums:?}");
    }

    #[test]
    fn test5() {
        let mut nums = vec![1, 3, 2, 4];
        Solution::next_permutation(&mut nums);
        println!("{nums:?}");
    }

    #[test]
    fn test6() {
        let mut nums = vec![1, 2, 4, 3];
        Solution::next_permutation(&mut nums);
        println!("{nums:?}");
    }

    #[test]
    fn test7() {
        let mut nums = vec![2, 3, 1];
        Solution::next_permutation1(&mut nums);
        println!("{nums:?}"); // excepted [3, 1, 2]
    }

    #[test]
    fn test8() {
        let mut nums = vec![2, 3, 1];
        Solution::next_permutation2(&mut nums);
        println!("{nums:?}"); // excepted [3, 1, 2]
    }

    #[test]
    fn test_zip() {
        for (i, j) in (0..90).zip(0..10) {
            print!("{} {} ", i, j);
        }
    }

    #[test]
    fn test_spilt_at_mut() {
        let mut a = [1, 2, 3, 4, 5];
        if let (left, right) = a.split_at_mut(2) { // idx is two, will be split to right part
            println!("{left:?}");
            println!("{right:?}");
        }
    }

    #[test]
    fn test_min_by_key() {
        let mut a = [1, 2, 3, 4, 5];
        let val = 4;
        a.iter_mut().min_by_key(|a| **a > val).unwrap();
    }
}


