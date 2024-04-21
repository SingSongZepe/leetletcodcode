use std::ptr::null_mut;

struct Solution;

impl Solution {
    // time exceed, O(n^3 + nlogn)    // O(n^2)
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        // if nums[0] + nums[1] + nums[2] > target {
        //     return nums[0] + nums[1] + nums[2];
        // } else if nums[nums.len()-3] + nums[nums.len()-2] + nums[nums.len()-1] < 0 {
        //     return nums[nums.len()-3] + nums[nums.len()-2] + nums[nums.len()-1];
        // }

        let mut ans = nums[0] + nums[1] + nums[2];
        let len = nums.len();

        for i in 0..(nums.len()-2) {
            if i > 0 && nums[i] == nums[i-1] {
                continue;
            }

            let (mut j, mut k) = (i+1, len-1);

            while j < k {
                let temp = nums[i] + nums[j] + nums[k];
                if (temp-target).abs() < (ans-target).abs() {
                    ans = temp;
                }

                if temp  < target {
                    while j<k+1 && j < len-2 && nums[j+2] == nums[j]  {j+=1;}
                    while j<k+1 && k > 2 && nums[k-2] == nums[k]  {k-=1;}
                    j+=1;
                }
                else if temp > target {
                    while j<k+1 && j < len-2 && nums[j+2] == nums[j]  {j+=1;}
                    while j<k+1 && k > 2 && nums[k-2] == nums[k]  {k-=1;}
                    k-=1;
                }
                else {
                    return  target;
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
        let nums = vec![-1, 2, 1, -4];
        let target = 1;
        let results = Solution::three_sum_closest(nums, target);
        println!("t1 {results}");
    }

    #[test]
    fn test2() {
        let nums = vec![0,0,0];
        let target = 1;
        let results = Solution::three_sum_closest(nums, target);
        println!("t2 {results}");
    }

    #[test]
    fn test3() {
        let nums = vec![-1000,-5,-5,-5,-5,-5,-5,-1,-1,-1];
        let target = -14;
        let results = Solution::three_sum_closest(nums, target);
        println!("t3 {results}");
    }
}

