mod test;

struct NumArray {
	nums: Vec<i32>
}

impl NumArray {
	// bad solution
	fn new(nums: Vec<i32>) -> Self {
		Self {
			nums
		}
	}
	fn sum_range(&self, left: i32, right: i32) -> i32 {
		let mut sum = 0;
		for i in left as usize..=right as usize {
			sum += self.nums[i];
		}
		sum
	}

	// fn sum_range(&self, left: i32, right: i32) -> i32 {
	// 	self.nums.iter().skip(left as usize).take((right - left + 1) as usize).sum()
	// }
}



struct NumArray1 {
	nums: Vec<i32>
}

impl NumArray1 {
	// fn new(nums: Vec<i32>) -> Self {
	// 	Self {
	// 		nums: nums.into_iter().fold((vec![], 0), |(mut vec, sum),(num)| {
	// 			vec.push(sum + num);
	// 			(vec, sum + num)
	// 		}).0
	// 	}
	// }
	fn new(nums: Vec<i32>) -> Self {
		let mut acc = 0;
		let v = nums.into_iter().map(|n| {
			acc += n;
			acc
		}).collect();
		Self {
			nums: v
		}
	}
	fn sum_range(&self, left: i32, right: i32) -> i32 {
		if left == 0 {
			return self.nums[right as usize];
		}
		self.nums[right as usize] - self.nums[left as usize - 1]
	}
}


struct NumArray2 {
	nums: Vec<i32>
}

impl crate::NumArray2 {
	fn new(mut nums: Vec<i32>) -> Self {
		let mut acc = 0;
		nums.insert(0, 0);
		let v = nums.into_iter().map(|n| {
			acc += n;
			acc
		}).collect();
		Self {
			nums: v
		}
	}
	fn sum_range(&self, left: i32, right: i32) -> i32 {
		self.nums[right as usize + 1] - self.nums[left as usize]
	}
}

fn main() {
    println!("Hello, world!");
}
