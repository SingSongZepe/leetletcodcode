
mod test;

struct Solution;

impl Solution {
	pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
		grumpy.windows(minutes as usize).enumerate().map(|(i, w)| {
			w.iter().enumerate().fold(0, |acc, (j, &g)| acc + g * customers[i + j])
		}).fold(0, |acc, x| {
			acc.max(x)
		}) + (0..customers.len()).map(|i| {
			if grumpy[i] == 0 {
				customers[i]
			} else {
				0
			}
		}).sum::<i32>()
	}
}


struct Solution1;

impl Solution1 {
	pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
		*(minutes as usize..customers.len()).fold([(0..minutes as usize).map(|i| if grumpy[i] == 1 { customers[i] } else { 0 }).sum::<i32>(), 0],
		|mut s, i| {
			s[1] = s[1].max(s[0]);
			s[0] += if grumpy[i] == 1 { customers[i] } else { 0 } - if grumpy[i-minutes as usize] == 1 { customers[i-minutes as usize] } else { 0 };
			s
		}).iter().max().unwrap()
		+ (0..customers.len()).map(|i| {
			if grumpy[i] == 0 {
				customers[i]
			} else {
				0
			}
		}).sum::<i32>()
	}
}

fn main() {
    println!("Hello, world!");
}
