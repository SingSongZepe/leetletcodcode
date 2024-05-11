mod test;

struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut histogram = vec![0; matrix[0].len()];
        let mut result = 0;
        for row in matrix {
            for (i, &c) in row.iter().enumerate() {
                match c {
                    '0' => histogram[i] = 0,
                    '1' => histogram[i] += 1,
                    _ => { panic!("Invalid input") }
                }
            }
            result = result.max(Self::find_max_rectangle(&histogram));
        }
        result
    }
    // monotonic stack solution
    fn find_max_rectangle(histogram: &Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = Vec::new();
        let mut result = 0;
        for (idx, &h) in histogram.iter().chain(&[0]).enumerate() {
            while let Some(&top) = stack.last() {
                if h > histogram[top] {
                    break;
                }
                let he = histogram[stack.pop().unwrap()]; // histogram[top]
                let wi = idx as i32- if let Some(&i) = stack.last() {i as i32} else {-1} - 1;
                result = result.max(he * wi);
            }
            stack.push(idx);
        }
        result
    }
}

fn helper(v: Vec<Vec<&str>>) -> Vec<Vec<char>> {
    let mut vec = Vec::new();
    for vr in v {
        let mut row = Vec::new();
        for i in vr {
            row.push(i.chars().nth(0).unwrap());
        }
        vec.push(row);
    }
    vec
}


fn main() {
    println!("Hello, world!");
}
