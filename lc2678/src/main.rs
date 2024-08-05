use std::f32::consts::E;

mod test;

struct Solution;
struct Solution1;

impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details.into_iter().fold(0, | acc, detail | {
			if (detail.chars().nth(11).unwrap() as u32 - 48) * 10 + 
			(detail.chars().nth(12).unwrap() as u32 - 48) > 60 {
				acc + 1
			} else {
				acc
			}
		})
    }
}


impl Solution1 {  
    pub fn count_seniors(details: Vec<String>) -> i32 {  
        details.into_iter().filter(|detail| {  
            (detail.chars().nth(11).unwrap() as i32 - '0' as i32) * 10 +  
            (detail.chars().nth(12).unwrap() as i32 - '0' as i32) > 60
        }).count() as i32 
    }  
}  

fn main() {
    println!("Hello, world!");
}
