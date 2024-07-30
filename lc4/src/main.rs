struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len1 = nums1.len();
        let len2 = nums2.len();

        let mut v: Vec<i32> = Vec::with_capacity(len1 + len2);

        for _ in 0..len1 + len2 {
            v.push(0);
        }

        let mut left1: usize = 0;
        let mut left2: usize = 0;

        while (left1 < len1 && left2 < len2) {
            if nums1[left1] < nums2[left2] {
                v[left1 + left2] = nums1[left1];
                left1 += 1;
            } else {
                v[left1 + left2] = nums2[left2];
                left2 += 1;
            }
        }

        if left1 >= len1 {
            for idx in left2..len2 {
                v[left1 + idx] = nums2[idx];
            }
        } else {
            for idx in left1..len1 {
                v[left2 + idx] = nums1[idx];
            }
        }

        if ((len1 + len2) % 2 == 0) {
            ((v[((len1 + len2) / 2 - 1)] + v[((len1 + len2) / 2)]) as f64) / 2.
        } else {
            v[((len1 + len2 - 1) / 2)] as f64
        }
    }

    // second, attempt to optimize time
    pub fn find_median_sorted_arrays1(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len1 = nums1.len();
        let len2 = nums2.len();

        let mut v: Vec<i32> = vec![0; len1 + len2];

        let mut left1: usize = 0;
        let mut left2: usize = 0;

        while (left1 < len1 && left2 < len2) {
            if nums1[left1] < nums2[left2] {
                v[left1 + left2] = nums1[left1];
                left1 += 1;
            } else {
                v[left1 + left2] = nums2[left2];
                left2 += 1;
            }
        }

        if left1 >= len1 {
            let slice = &nums2[left2..len2];
            v.splice(left1 + 1..(left1 + slice.len() + 1), slice.iter().cloned());
        } else {
            let slice = &nums1[left1..len1];
            v.splice(left2 + 1..(left2 + slice.len() + 1), slice.iter().cloned());
        }

        if ((len1 + len2) % 2 == 0) {
            ((v[(len1 + len2) / 2 - 1] + v[(len1 + len2) / 2]) as f64) / 2.
        } else {
            v[(len1 + len2 - 1) / 2] as f64
        }
    }
}

fn main() {
    println!("Hello, world!");
    println!("{}", Solution::find_median_sorted_arrays1(vec![1, 3], vec![2]));
}
