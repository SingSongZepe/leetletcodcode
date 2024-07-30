mod test;

struct Solution;

impl Solution {
	pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
		macro_rules! mkusize {
			($i: tt) => { $i as usize };
		}

		let (mut i, mut j, mut r): (i32, i32, i32) = (m - 1 , n - 1, m + n - 1);
		while i  >= 0 && j >= 0 {
			if nums1[mkusize!(i)] < nums2[mkusize!(j)] {
				nums1[mkusize!(r)] = nums2[mkusize!(j)];
				r -= 1;
				j -= 1;
			} else {
				nums1[mkusize!(r)] = nums1[mkusize!(i)];
				r -= 1;
				i -= 1;
			}
		}
		// if there are still elements in nums2, copy them to nums1
		while j >= 0 {
			nums1[mkusize!(r)] = nums2[mkusize!(j)];
			r -= 1;
			j -= 1;
		}
	}
}

fn main() {
    println!("Hello, world!");
}
