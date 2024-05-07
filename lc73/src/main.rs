
struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut clear_rows = vec![];
        let mut clear_cols = vec![];
        let mut zero_rows = vec![false; matrix.len()];
        let mut zero_cols = vec![false; matrix[0].len()];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 0 {
                    if !zero_rows[i] {
                        clear_rows.push(i);
                        zero_rows[i] = true;
                    }
                    if !zero_cols[j] {
                        clear_cols.push(j);
                        zero_cols[j] = true;
                    }
                }
            }
        }
        // clear the rows and columns
        for i in clear_rows {
            for j in 0..matrix[0].len() {
                matrix[i][j] = 0;
            }
        }
        for j in clear_cols {
            for i in 0..matrix.len() {
                matrix[i][j] = 0;
            }
        }
    }
}

fn print_matrix(matrix: &Vec<Vec<i32>>) {
    for row in matrix {
        println!("{:?}", row);
    }
    println!("");
}

#[cfg(test)]
mod tests {
    use crate::{
        Solution,
        print_matrix,
    };

    #[test]
    fn test1() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        Solution::set_zeroes(&mut matrix);
        println!("{:?}", matrix); // expected output: [[1, 0, 1], [0, 0, 0], [1, 0, 1]]
    }

    #[test]
    fn test2() {
        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        Solution::set_zeroes(&mut matrix);
        println!("{:?}", matrix); // expected output: [[0, 0, 0, 0], [0, 4, 5, 0], [0, 3, 1, 0]]
    }

    #[test]
    fn test3() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        Solution::set_zeroes(&mut matrix);
        println!("{:?}", matrix);
    }
}

fn main() {
    println!("Hello, world!");
}
