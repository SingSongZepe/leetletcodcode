
struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {

        false
    }
}

pub fn helper(strs: Vec<&str>) -> Vec<char> {
    let mut chars = vec![];
    for str in strs {
        if let Some(char) = str.chars().next() {
            chars.push(char)
        }
    }
    chars
}

#[cfg(test)]
mod tests {
    use crate::{helper, Solution};

    #[test]
    fn test1() {
        let board =
            vec![helper(["5","3",".",".","7",".",".",".","."].to_vec()).to_vec()
            ,helper(["6",".",".","1","9","5",".",".","."].to_vec()).to_vec()
            ,helper([".","9","8",".",".",".",".","6","."].to_vec()).to_vec()
            ,helper(["8",".",".",".","6",".",".",".","3"].to_vec()).to_vec()
            ,helper(["4",".",".","8",".","3",".",".","1"].to_vec()).to_vec()
            ,helper(["7",".",".",".","2",".",".",".","6"].to_vec()).to_vec()
            ,helper([".","6",".",".",".",".","2","8","."].to_vec()).to_vec()
            ,helper([".",".",".","4","1","9",".",".","5"].to_vec()).to_vec()
            ,helper([".",".",".",".","8",".",".","7","9"].to_vec()).to_vec()];
        let result = Solution::is_valid_sudoku(board);
        println!("{}", result);
    }
}

fn main() {
    println!("Hello, world!");
}

