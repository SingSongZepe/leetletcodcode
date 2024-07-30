use std::ptr::null_mut;

struct Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        candidates.sort_unstable();
        Solution::combine(&candidates, target, &mut vec![], &mut result, 0);
        result
    }

    pub fn combine(candidates: &Vec<i32>, target: i32, select: &mut Vec<i32>, result: &mut Vec<Vec<i32>>, from: usize) {
        if target == 0 {
            result.push(select.clone());
            return;
        }
        if target < 0 { return }

        for idx in from..candidates.len() {
            if from != idx && candidates[idx] == candidates[idx - 1] { continue }
            select.push(candidates[idx]);
            Solution::combine(candidates, target - candidates[idx], select, result, idx + 1);
            select.pop(); // !
        }
    }
}

impl Solution {
    pub fn combination_sum21(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut ans = Vec::new();
        Solution::backtrack(&candidates, 0, target, &mut Vec::new(), &mut ans);
        ans
    }

    fn backtrack(nums: &[i32], i: usize, target: i32, cur: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if target == 0 {
            ans.push(cur.clone());
            return;
        }
        if target < 0 { return }

        for j in i..nums.len() {
            if i != j && nums[j] == nums[j - 1] { continue }
            cur.push(nums[j]);
            Solution::backtrack(nums, j + 1, target - nums[j], cur, ans);
            cur.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use std::f64::consts::PI;

    #[test]
    fn test1() {
        let candidates = vec![10,1,2,7,6,1,5];
        let target = 8;
        let result = Solution::combination_sum2(candidates, target);
        println!("{result:?}");
        // excepted [
        //     [1,1,6],
        //     [1,2,5],
        //     [1,7],
        //     [2,6]
        // ]
    }

    #[test]
    fn test2() {
        let candidates = vec![2,5,2,1];
        let target = 5;
        let result = Solution::combination_sum2(candidates, target);
        println!("{result:?}");
        // excepted [
        //     [1,2,2],
        //     [5]
        // ]
    }

    #[test]
    fn test3() {
        let candidates = vec![2,2,4];
        let target = 4;
        let result = Solution::combination_sum2(candidates, target);
        println!("{result:?}");
        // excepted [
        //     [1,2,2],
        //     [5]
        // ]
    }

    #[test]
    fn test4() {
        let candidates = vec![2,5,2,1];
        let target = 5;
        let result = Solution::combination_sum21(candidates, target);
        println!("{result:?}");
        // excepted [
        //     [1,2,2],
        //     [5]
        // ]
    }

    #[test]
    fn test5() {
        let candidates = vec![2,2,4];
        let target = 4;
        let result = Solution::combination_sum21(candidates, target);
        println!("{result:?}");
        // excepted [
        //     [1,2,2],
        //     [5]
        // ]
    }

    #[test]
    fn test6() {
        let candidates = vec![2,2,2];
        let target = 6;
        let result = Solution::combination_sum21(candidates, target);
        println!("{result:?}");
        // excepted [
        //     [2,2,2],
        // ]
    }

    #[test]
    fn test_slices() {
        fn print_v(v: &[i32]) {
            for i in v {
                print!("{} ", *i);
            }
        }

        let v = vec![1, 2, 3, 4];
        print_v(&v);
        let mut v1 = v.clone();
        v1.remove(1);
        print_v(&v1);
    }

    #[test]
    fn test_sort_unstable() {
        let mut v = vec![10, 3, 4, 5, 9];
        v.sort_unstable();
        println!("{v:?}");

        let mut v1 = vec![10.0, 3.0, 4.0, 5.0, 9.0];
        v1.sort_by(|a, b| {
            let sin_a = (*a as f64).to_degrees().sin();
            let sin_b = (*b as f64).to_degrees().sin();
            sin_a.partial_cmp(&sin_b).unwrap()
        });

    }

}

fn main() {
    println!("Hello, world!");
}
