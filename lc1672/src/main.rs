mod test;

struct Solution;

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts
            .iter()
            .map(|account| account.iter().sum())
            .max()
            .unwrap_or_default()
    }
}

fn main() {
    println!("Hello, world!");
}
