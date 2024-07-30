mod test;

struct Solution;

trait Summary<T> {
	fn summarize(&self) -> i32;
}

impl Summary<i32> for (i32, i32) {
	fn summarize(&self) -> i32 {
		return self.0 + self.1;
	}
}


impl Solution {
	pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
		let s = prices.into_iter().fold((i32::MAX, i32::MAX), |(min1, min2), price| {
			if price < min2 {
				(min2, price)
			} else if price < min1 {
				(price, min2)
			} else {
				(min1, min2)
			}
		}).summarize();
		if s <= money {
			money - s
		} else {
			money
		}
	}
}

struct Solution1;

impl Solution1 {
	pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
		if prices.iter().fold((i32::MAX, i32::MAX), |(min1, min2), &price| if price < min2 {(min2, price)} else if price < min1 {(price, min2)} else {(min1, min2)}).summarize() <= money {money - prices.iter().fold((i32::MAX, i32::MAX), |(min1, min2), &price| if price < min2 {(min2, price)} else if price < min1 {(price, min2)} else {(min1, min2)}).summarize()} else {money}
	}
}

fn main() {
    println!("Hello, world!");
}
