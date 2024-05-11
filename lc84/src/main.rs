mod test;

 struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut mstack: Vec<usize> = vec![]; // monotonic stack

        for (idx, &h) in heights.iter().chain(&[0]).enumerate() { // there chain() is a very smart way to solve the last height
            while let Some(&top) = mstack.last() {
                if h > heights[top] {
                    break;
                }
                // calculate the area of the rectangle with heights[top] and h
                let he = heights[mstack.pop().unwrap()];
                // if v is empty, then wi means from 0
                // if v has element, means
                let wi = idx as i32 - if let Some(&i) = mstack.last() {i as i32} else { -1 } - 1;
                result = result.max(he * wi);
            }
            mstack.push(idx);
        }
        result
    }
}

struct Solution1;

impl Solution1 {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        heights
            .iter()
            .chain(&[0])
            .enumerate()
            .fold((vec![], 0),
                  |(mut v, mut ans), (i, &x)| {
                      while let Some(&idx) = v.last() {
                          if x > heights[idx] {break;}
                          let height = heights[v.pop().unwrap()];
                          let tmp: i32 = if let Some(&i) = v.last() {i as i32} else {-1};
                          let weight = i as i32 - tmp - 1;
                          ans = ans.max(height * weight);
                      }
                      v.push(i);
                      (v, ans)
                  })
            .1
    }
}

fn main() {
    println!("Hello, world!");
}
