mod test;

struct Solution;

impl Solution {
	pub fn min_difference(nums: Vec<i32>) -> i32 {
		if nums.len() < 5 {
			return 0;
		}

		let mut larges = [i32::MIN; 4];
		let mut smalls =  [i32::MAX; 4];

		for &num in &nums {
			if num > larges[0] {
				larges[3] = larges[2];
				larges[2] = larges[1];
				larges[1] = larges[0];
				larges[0] = num;
			} else if num > larges[1] {
				larges[3] = larges[2];
				larges[2] = larges[1];
				larges[1] = num;
			} else if num > larges[2] {
				larges[3] = larges[2];
				larges[2] = num;
			} else if num > larges[3] {
				larges[3] = num;
			}

			if num < smalls[0] {
				smalls[3] = smalls[2];
				smalls[2] = smalls[1];
				smalls[1] = smalls[0];
				smalls[0] = num;
			} else if num < smalls[1] {
				smalls[3] = smalls[2];
				smalls[2] = smalls[1];
				smalls[1] = num;
			} else if num < smalls[2] {
				smalls[3] = smalls[2];
				smalls[2] = num;
			} else if num < smalls[3] {
				smalls[3] = num;
			}
		}
		larges.into_iter().zip(smalls.into_iter().rev()).map(|(l, s)| l - s).min().unwrap()
	}
}

struct Solution1;

impl Solution1 {
	pub fn min_difference(nums: Vec<i32>) -> i32 {
		if nums.len() < 5 {
			return 0;
		}

		let mut largest = vec![i32::MIN; 4];
		let mut smallest = vec![i32::MAX; 4];

		for &num in &nums {
			for i in 0..4 {
				if num > largest[i] {
					for j in (i + 1..4).rev() {
						largest[j] = largest[j - 1];
					}
					largest[i] = num;
					break;
				}
			}

			for i in 0..4 {
				if num < smallest[i] {
					for j in (i + 1..4).rev() {
						smallest[j] = smallest[j - 1];
					}
					smallest[i] = num;
					break;
				}
			}
		}

		largest.into_iter().zip(smallest.into_iter().rev()).map(|(l, s)| l - s).min().unwrap()
	}
}

fn main() {
    println!("Hello, world!");
}
