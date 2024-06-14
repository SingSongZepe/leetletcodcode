mod test;

struct Solution;

impl Solution {
	pub fn gcd_of_strings(str1: String, str2: String) -> String {
		let str1 = str1.as_bytes();
		let str2 = str2.as_bytes();
		let (l1, l2) = (str1.len(), str2.len());
		'lp:
		for i in (0..l1.min(l2)).rev() {
			if l1 % (i+1) != 0 || l2 % (i+1) != 0 {
				continue;
			}
			let substring = &str1[..i+1];
			for j in 0..l1/(i+1) {
				if &str1[j*(i+1)..(j+1)*(i+1)] != &substring[0..i+1] {
					continue 'lp;
				}
			}
			for j in 0..l2/(i+1) {
				if &str2[j*(i+1)..(j+1)*(i+1)] != &substring[0..i+1] {
					continue 'lp;
				}
			}
			return String::from_utf8(substring.to_vec()).unwrap();
		}
		"".to_string()
	}
}

fn main() {
    println!("Hello, world!");
}
