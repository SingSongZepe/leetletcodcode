use log::log;

struct Solution;

use std::cmp::max;
// O(n)
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut mc = nums[0];
        let mut mt = nums[0];
        for i in nums.iter().skip(1) {
            mc = if mc < 0 { *i } else { *i + mc };
            // mc = *i + mc;
            mt = mc.max(mt);
        }
        mt
    }
}

// O(n)
impl Solution {
    pub fn max_sub_array1(nums: Vec<i32>) -> i32 {
        // Kadane Algo
        let mut max_global = nums[0];
        let mut max_current = nums[0];

        for &num in nums.iter().skip(1){
            max_current = num.max(max_current + num);
            max_global = max_global.max(max_current);
        }
        max_global
    }
}

// divide and conquer
// O(logn)
impl Solution {
    pub fn max_sub_array2(nums: Vec<i32>) -> i32 {
        return max_sub_array_sum(&nums, 0, nums.len()-1);
    }
}

fn max_sum_crossing(nums: &Vec<i32>, mid: usize, lo: usize, hi: usize) -> i32 {
    let mut bestLeft: i32 = i32::MIN;
    let mut v: i32 = 0;
    for i in (lo..=mid).rev() {
        v = v + nums[i];
        bestLeft =  max(v, bestLeft);
    }
    let mut bestRight: i32 = i32::MIN;
    v = 0;
    for i in (mid+1..=hi) {
        v = v + nums[i];
        bestRight = max(v, bestRight);
    }
    return bestLeft + bestRight;
}

fn max_sub_array_sum(nums: &Vec<i32>, lo: usize, hi: usize) -> i32 {
    if(hi == lo) {
        return nums[lo];
    }
    let mid = (lo + hi) / 2;
    let leftSum = max_sub_array_sum(nums, lo, mid);
    let rightSum = max_sub_array_sum(nums, mid+1, hi);
    let crossingSum = max_sum_crossing(nums, mid, lo, hi);
    // let crossingSum = i32::MIN;
    return max(max(leftSum, rightSum), crossingSum);
}

impl Solution {
    pub fn max_sub_array3(nums: Vec<i32>) -> i32 {
        *Solution::ac_msa(&nums[..]).iter().max().unwrap()
    }

    pub fn ac_msa(nums: &[i32]) -> [i32; 4] {
        /*
        * The all contiguous msa returns the max sub arrays for each of the 4
        * sub array segments:
        *   1. l/r contiguous
        *   2. left contiguous
        *   3. right contiguous
        *   4. neither contiguous
        * This allows us to pick and choose the optimal max sub array depending on continuity.
        */
        // Base case return the single element for all 4 segments
        if nums.len() == 1 {
            return [nums[0]; 4];
        }
        // Otherwise, split the array in two and calculate their msa
        let (left_nums, right_nums) = nums.split_at(nums.len()/2);
        let [l_bc, l_lc, l_rc, l_nc] = Solution::ac_msa(left_nums);
        let [r_bc, r_lc, r_rc, r_nc] = Solution::ac_msa(right_nums);
        // Now calculate the 4 segments for the combined arrays as follows:
        // 1. Contiguous on both sides will be the sum of the sub arrays that are contiguous
        // on both sides
        let this_bc = l_bc + r_bc;
        // 2. The left contigous segment is the max between the left side's left contiguous
        // AND the left side's both contigous + right side's left contiguous.
        // This covers the case where the strictly left side's left contiguous max is less
        // than taking the entire left side adding the right side's left contiguous max.
        let this_lc = max(l_lc, l_bc + r_lc);
        // 3. Same as the left contiguous but with sides reversed.
        let this_rc = max(r_rc, r_bc + l_rc);
        // 4. For the discontiguous or non-contiguous segment it is simply the max between
        // the non-contiguous segments for both sides and the middle contiguous sum.
        let this_nc = max(max(r_nc, l_nc), l_rc + r_lc);
        return [this_bc, this_lc, this_rc, this_nc];
    }
}


fn helper(arr: &[i32]) -> Vec<i32> {
    let mut v = vec![];
    for i in arr {
        v.push(*i);
    }
    v
}

#[cfg(test)]
mod tests {
    use crate::{helper, Solution};

    #[test]
    fn test1() {
        let nums = vec![-2,1,-3,4,-1,2,1,-5,4];
        let result = Solution::max_sub_array(nums);
        println!("{}", result); // excepted 6
    }

    #[test]
    fn test2() {
        let nums = vec![1];
        let result = Solution::max_sub_array(nums);
        println!("{}", result); // 1
    }

    #[test]
    fn test3() {
        let nums = vec![5,4,-1,7,8];
        let result = Solution::max_sub_array(nums);
        println!("{}", result); // 23
    }

    #[test]
    fn test11() {
        let nums = vec![-2,1,-3,4,-1,2,1,-5,4];
        let result = Solution::max_sub_array1(nums);
        println!("{}", result); // excepted 6
    }

    #[test]
    fn test21() {
        let nums = vec![1];
        let result = Solution::max_sub_array1(nums);
        println!("{}", result); // 1
    }

    #[test]
    fn test31() {
        let nums = vec![5,4,-1,7,8];
        let result = Solution::max_sub_array1(nums);
        println!("{}", result); // 23
    }

    #[test]
    fn test12() {
        let nums = vec![-2,1,-3,4,-1,2,1,-5,4];
        let result = Solution::max_sub_array2(nums);
        println!("{}", result); // excepted 6
    }

    #[test]
    fn test22() {
        let nums = vec![1];
        let result = Solution::max_sub_array2(nums);
        println!("{}", result); // 1
    }

    #[test]
    fn test32() {
        let nums = vec![5,4,-1,7,8];
        let result = Solution::max_sub_array2(nums);
        println!("{}", result); // 23
    }


    #[test]
    fn test13() {
        let nums = vec![-2,1,-3,4,-1,2,1,-5,4];
        let result = Solution::max_sub_array3(nums);
        println!("{}", result); // excepted 6
    }

    #[test]
    fn test23() {
        let nums = vec![1];
        let result = Solution::max_sub_array3(nums);
        println!("{}", result); // 1
    }

    #[test]
    fn test33() {
        let nums = vec![5,4,-1,7,8];
        let result = Solution::max_sub_array3(nums);
        println!("{}", result); // 23
    }

    #[test]
    fn test_array() {
        let i = [1, 2, 3, 4];
        println!("{:?}", helper(&i[..]))
    }
}



fn main() {
    println!("Hello, world!");
}
