struct Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let len = nums.len();
        if len < 4 {
            return vec![];
        }

        nums.sort();
        // if (nums[0] + nums[1] + nums[2] + nums[3]) as i64 > target a ||
        //     nums[nums.len()-1] + nums[nums.len()-2] + nums[nums.len()-3] + nums[nums.len()-4] < target  {
        //     return vec![];
        // }

        let mut ans: Vec<Vec<i32>> = Vec::new();

        for p1 in 0..(len-3) { // p1
            if p1 > 0 && nums[p1] == nums[p1-1] {
                continue;
            }
            for p2 in p1+1..(len-2) { // p2
                if p2 > p1 + 1 && nums[p2] == nums[p2-1] {
                    continue;
                }
                let (mut j, mut k) = (p2 + 1, len - 1);

                while j < k {
                    let sum: i64 = nums[j]  as i64 + nums[k]  as i64 + nums[p1]  as i64 + nums[p2] as i64;
                    if sum > i32::MAX as i64 || sum < i32::MIN as i64 {
                        j += 1;
                        k -= 1;
                        continue;
                    }
                    if sum as i32 == target {
                        ans.push(vec![nums[p1], nums[p2], nums[j], nums[k]]);
                        while j < k && nums[j + 1] == nums[j] { j += 1; }
                        while j < k && nums[k - 1] == nums[k] { k -= 1; }
                        j += 1;
                        k -= 1;
                    } else if (sum as i32) < target {
                        j += 1;
                    } else {
                        k -= 1;
                    }
                }
            }
        }
        ans
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
        let nums = vec![1,0,-1,0,-2,2];
        let target = 0;
        let result = Solution::four_sum(nums, target);
        println!("t1 {result:?}");
    }

    #[test]
    fn test2() {
        let nums = vec![2,2,2,2,2];
        let target = 8;
        let result = Solution::four_sum(nums, target);
        println!("t2 {result:?}");
    }

    #[test]
    fn test3() {
        let nums = vec![0,0,0,1000000000,1000000000,1000000000,1000000000];
        let target = 1000000000;
        let result = Solution::four_sum(nums, target);
        println!("t3 {result:?}");
    }
}