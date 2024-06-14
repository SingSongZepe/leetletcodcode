mod test;

struct Solution;

// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
	pub fn first_bad_version(&self, n: i32) -> i32 {
		let (mut l, mut r) = (0, n);
		while l < r {
			let mid = ((l as i64 + r as i64) / 2) as i32;
			if self.isBadVersion(mid) {
				r = mid;
			} else {
				l = mid + 1;
			}
		}
		l
	}
}

impl Solution {
	pub fn new() -> Self {
		Solution {}
	}
	pub fn isBadVersion(&self, version: i32) -> bool {
		version >= 1702766719
	}
}

fn main() {
    println!("Hello, world!");
}
