
struct Solution;

impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {

        let mut results = vec![];
        for l in 0..arr.len()-1 {
            for r in (l+1..arr.len()).rev() {
                results.push(vec![arr[l], arr[r]]);
            }
        }
        results.sort_by(|a, b| {
            let p1 = (a[0] as f64) / (a[1] as f64);
            let p2 = (b[0] as f64) / (b[1] as f64);
            p1.partial_cmp(&p2).unwrap()
        });
        results[k as usize-1].clone()
    }
}

struct Solution1;

use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::cmp::Ord;


#[derive(Debug)]
struct Ratio(i32, i32);

impl Ratio {
    fn value(&self) -> f32 {
        self.0 as f32 / self.1 as f32
    }
    fn to_vec(&self) -> Vec<i32> {
        vec![self.0, self.1]
    }
}

impl Ord for Ratio {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().partial_cmp(&other.value()).unwrap_or(Ordering::Equal)
    }
}

impl Eq for Ratio {}

impl PartialEq<Self> for Ratio {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution1 {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut bh: BinaryHeap<Ratio> = BinaryHeap::new();
        for i in 0..arr.len()-1 {
            for j in i + 1..arr.len() {
                // let p = (arr[i] as f64) / (arr[j] as f64);
                bh.push(Ratio(arr[i], arr[j]));
            }
        }

        let count = bh.len();
        for _ in 0..count-k as usize {
            bh.pop();
        }
        bh.pop().unwrap().to_vec()
    }
}

struct Solution2;

impl Solution2 {
    // ! binary search, best solution
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut k = k as usize;
        let arr2: Vec<f64> = arr.iter().map(|&x| x as f64).collect();
        let mut lo = 0.0;
        let mut hi = 1.0;
        let mut v: Vec<(usize, usize, usize)> = (1..arr.len())
            .map(|i| (0, i, i))
            .collect();

        while v.len() != 1 {
            let mid = (lo + hi) * 0.5;
            let mut count = 0;
            let mut res = Vec::<usize>::with_capacity(v.len());
            for i in 0..v.len() {
                let (left, right, denom) = v[i];
                let split_at = mid * arr2[denom];
                let j = arr2[left..right].partition_point(|&x| x < split_at);
                res.push(j + left);
                count += j;
            }

            if count < k {
                let mut i = 0;
                while i < v.len() { // trim the 2D counting range.
                    if res[i] == v[i].1 {
                        v.swap_remove(i);
                        res.swap_remove(i);
                    }
                    else {
                        v[i].0 = res[i];
                        i += 1;
                    }
                }
                k -= count;
                lo = mid;
            }
            else {
                let mut i = 0;
                while i < v.len() { // trim the 2D counting range.
                    if res[i] == v[i].0 {
                        v.swap_remove(i);
                        res.swap_remove(i);
                    }
                    else {
                        v[i].1 = res[i];
                        i += 1;
                    }
                }
                hi = mid;
            }
        }
        let (left, _, denom) = v[0];
        vec![arr[left + k - 1], arr[denom]]
    }
}

struct Solution3;

impl Solution3 {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let arr2: Vec<f64> = arr.iter().map(|&x| x as f64).collect();
        let (numerator, denominator) = Self::find_kth_fraction(&arr2, k as usize, 0.0, 1.0);
        vec![arr[numerator], arr[denominator]]
    }

    fn find_kth_fraction(arr: &Vec<f64>, k: usize, lo: f64, hi: f64) -> (usize, usize) {
        let mid = (lo + hi) * 0.5;
        let mut count = 0;
        let mut res = Vec::<usize>::with_capacity(arr.len());
        for i in 0..arr.len() {
            let split_at = mid * arr[i];
            let j = arr.partition_point(|&x| x < split_at);
            res.push(j);
            count += j;
        }

        if count < k {
            let mut new_arr = Vec::new();
            for i in 0..arr.len() {
                if res[i] < arr.len() {
                    new_arr.push(arr[res[i]]);
                }
            }
            Self::find_kth_fraction(&new_arr, k - count, mid, hi)
        } else {
            let mut new_arr = Vec::new();
            for i in 0..arr.len() {
                if res[i] > 0 {
                    new_arr.push(arr[res[i] - 1]);
                }
            }
            Self::find_kth_fraction(&new_arr, k, lo, mid)
        }
    }
}

struct Solution4;

impl Solution4 {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let (mut l, mut r) = (0f64, 1f64);
        let (mut p, mut q) = (0, 1);
        while l < r {
            let m = (l + r) / 2f64;
            let (mut cnt, mut j) = (0usize, 0usize);
            p = 0; q = 1;
            for i in 0..arr.len() {
                let tmp = arr[i] as f64 / m;
                while j < arr.len() && tmp > arr[j] as f64  {j+=1;}
                if j == arr.len() {break;}
                cnt += arr.len() - j;
                if p * arr[j] < q * arr[i] {
                    p=arr[i]; q=arr[j];
                }
            }

            if cnt == k as usize {break;}
            else if cnt < k as usize {l = m;}
            else {r = m;}
        }
        vec![p, q]
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BinaryHeap;
    use std::cmp::Ordering;
    use crate::{
        Solution,
        Solution1,
        Ratio,
        Solution2,
        Solution3,
    };

    #[test]
    fn test1() {
        let arr = vec![1, 2, 3, 5];
        let k = 3;
        let result = Solution::kth_smallest_prime_fraction(arr, k);
        println!("{:?}", result);
    }

    #[test]
    fn test2() {
        let arr = vec![1, 7];
        let k = 1;
        let result = Solution::kth_smallest_prime_fraction(arr, k);
        println!("{:?}", result);
    }

    #[test]
    fn test11() {
        let arr = vec![1, 2, 3, 5];
        let k = 3;
        let result = Solution1::kth_smallest_prime_fraction(arr, k);
        println!("{:?}", result);
    }

    #[test]
    fn test21() {
        let arr = vec![1, 7];
        let k = 1;
        let result = Solution1::kth_smallest_prime_fraction(arr, k);
        println!("{:?}", result);
    }

    #[test]
    fn test12() {
        let arr = vec![1, 2, 3, 5];
        let k = 3;
        let result = Solution2::kth_smallest_prime_fraction(arr, k);
        println!("{:?}", result);
    }

    #[test]
    fn test22() {
        let arr = vec![1, 7];
        let k = 1;
        let result = Solution2::kth_smallest_prime_fraction(arr, k);
        println!("{:?}", result);
    }

    #[test]
    fn test13() {
        let arr = vec![1, 2, 3, 5];
        let k = 3;
        let result = Solution3::kth_smallest_prime_fraction(arr, k);
        println!("{:?}", result);
    }

    #[test]
    fn test23() {
        let arr = vec![1, 7];
        let k = 1;
        let result = Solution3::kth_smallest_prime_fraction(arr, k);
        println!("{:?}", result);
    }


    #[test]
    fn test_ratio() {
        let r1 = Ratio(1, 2);
        let r2 = Ratio(2, 3);
        let r3 = Ratio(1, 3);
        if r1.cmp(&r2) == Ordering::Less {
            println!("less")
        } else {
            println!("not less")
        }
        if r1.cmp(&r3) == Ordering::Less {
            println!("less")
        } else {
            println!("not less")
        }
    }

    #[test]
    fn test_heap() {
        let mut bh: BinaryHeap<Ratio> = BinaryHeap::new();
        bh.push(Ratio(1, 2));
        bh.push(Ratio(2, 3));
        bh.push(Ratio(1, 3));
        for _ in 0..3 {
            let r = bh.peek().unwrap();
            println!("{:?}", r);
        }
    }

    #[test]
    fn test_swap_remove() {
        let mut v = vec![(0, 1, 2), (1, 2, 3), (2, 3, 4)];
        v.swap_remove(1);
        println!("{:?}", v);

        // 调用 swap_remove 方法后，它会返回被移除的元素，并且将被移除的元素的位置由最后一个元素填补，然后删除最后一个元素
        // move the last element to the position of the removed element
        // and remove the last element
        let mut v = vec![1, 2, 3, 4, 5];
        let removed_element = v.swap_remove(2);
        println!("Removed element: {}", removed_element);  // 输出被移除的元素
        println!("Updated vector: {:?}", v);  // 输出更新后的向量

    }
}


fn main() {
    println!("Hello, world!");
}
