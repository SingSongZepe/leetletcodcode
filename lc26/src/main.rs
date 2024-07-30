
struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        // maybe better with the capacity, but not
        let mut ns = Vec::with_capacity(nums.len());
        ns.push(nums[0]);
        let len = nums.len();

        for i in 1..len {
            if nums[i-1] == nums[i] {
                continue;
            }
            ns.push(nums[i]);
        }
        std::mem::swap(nums, &mut ns);

        nums.len() as i32
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
        let mut nums = vec![1, 1, 2, 3, 4, 4, 4, 5, 5, 5];
        let k = Solution::remove_duplicates(&mut nums);

        println!("{nums:?}");
        println!("{}", k);
    }
}