
struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len();
        Solution::rotate_edge(matrix, 0, &len);
    }

    pub fn rotate_edge(matrix: &mut Vec<Vec<i32>>, i: usize, len: &usize) {
        if (*len % 2 == 1 && i >= (*len - 1) / 2) || (*len % 2 == 0 && i >= *len / 2) { // || i >= *len / 2
            return;
        }
        // let N = len - 2 * i;
        for j in i..len-i-1 {
            Solution::swap(matrix, i, j, j, len-i-1);
            Solution::swap(matrix, i, j, len-i-1, len-j-1);
            Solution::swap(matrix, i, j, len-j-1, i);
        }
        Solution::rotate_edge(matrix, i + 1, len);
    }

    pub fn swap(matrix: &mut Vec<Vec<i32>>, r1: usize, c1: usize, r2: usize, c2: usize) {
        let tmp = matrix[r1][c1];
        matrix[r1][c1] = matrix[r2][c2];
        matrix[r2][c2] = tmp;
    }
}

pub fn print_matrix(matrix: &Vec<Vec<i32>>) {
    for line in matrix {
        println!("{line:?}");
    }
    println!();
}

#[cfg(test)]
mod tests {
    use crate::{print_matrix, Solution};

    #[test]
    fn test1() {
        let mut matrix =
                   [[5,1,9,11].to_vec(),
                    [2,4,8,10].to_vec(),
                    [13,3,6,7].to_vec(),
                    [15,14,12,16].to_vec()].to_vec();
        Solution::rotate(&mut matrix);
        print_matrix(&matrix);
    }

    #[test]
    fn test2() {
        let mut matrix = vec![[1,2,3].to_vec(),[4,5,6].to_vec(),[7,8,9].to_vec()];
        Solution::rotate(&mut matrix);
        print_matrix(&matrix);
    }

    #[test]
    fn test3() {
        let mut matrix = vec![[1].to_vec()];
        Solution::rotate(&mut matrix);
        print_matrix(&matrix);
    }
}

fn main() {
    println!("Hello, world!");
}
