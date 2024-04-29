
struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {

    }
}

pub fn helper(board: [[&str; 9]; 9]) -> Vec<Vec<char>> {
    let mut b = vec![];
    for line in board {
        let mut l = vec![];
        for cell in line {
            if let Some(char) = cell.chars().next() {
                l.push(char);
            }
        }
        b.push(l);
    }
    b
}

#[cfg(test)]
mod tests {
    use crate::{helper, Solution};

    #[test]
    fn test1() {
        let mut board = helper(
            [["8","3",".",".","7",".",".",".","."]
            ,["6",".",".","1","9","5",".",".","."]
            ,[".","9","8",".",".",".",".","6","."]
            ,["8",".",".",".","6",".",".",".","3"]
            ,["4",".",".","8",".","3",".",".","1"]
            ,["7",".",".",".","2",".",".",".","6"]
            ,[".","6",".",".",".",".","2","8","."]
            ,[".",".",".","4","1","9",".",".","5"]
            ,[".",".",".",".","8",".",".","7","9"]]
        );

        Solution::solve_sudoku(&mut board);
        println!("{board:?}");
    }

    #[test]
    fn test_2dv() {
        let _2dv = vec![vec![1,2],vec![3,4]];
        println!("{:?}", _2dv);
    }
}

fn main() {
    println!("Hello, world!");
}
