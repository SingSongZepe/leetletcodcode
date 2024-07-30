
struct Solution;

impl Solution {
    // scan line
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row = vec![];
        let mut chunks = vec![];
        let mut columns = vec![vec![false; 9]; 9]; // len = 9

        for (irow, line ) in board.iter().enumerate() {
            row = vec![false; 9];
            if irow % 3 == 0 {
                chunks = vec![vec![false; 9]; 3];
            }
            for (icolumn, cell) in line.iter().enumerate() {
                if *cell == '.' {
                    continue;
                }
                let val = *cell as usize - 49;
                if row[val] || chunks[icolumn / 3][val] || columns[icolumn][val] {
                    return false;
                } else {
                    row[val] = true;
                    chunks[icolumn / 3][val] = true;
                    columns[icolumn][val] = true;
                }
            }
        }
        true
    }
}


impl Solution {
    pub fn is_valid_sudoku1(board: Vec<Vec<char>>) -> bool {
        let block_it = |i, j| board[i..i + 3].iter().flat_map(move |row| &row[j..j + 3]);

        (0..3).all(|i| (0..3).all(|j| Solution::uniq(block_it(i * 3, j * 3))))
            && (0..9).all(|i| Solution::uniq(board.iter().map(|row| &row[i])))
            && board.iter().map(|v| v.iter()).all(Solution::uniq)
    }


    fn uniq<'a>(it: impl Iterator<Item = &'a char>) -> bool {
        let mut v = [0; 10];
        it.filter_map(|c| c.to_digit(10))
            .for_each(|c| v[c as usize] += 1);
        v.iter().all(|&v| v < 2) // <= 1
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

pub fn helper1(board: [[&str; 9]; 9]) -> Vec<Vec<char>> {
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
    use crate::{helper, helper1, Solution};

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

    #[test]
    fn test2() {
        let board = helper1(
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

        let result = Solution::is_valid_sudoku(board);
        println!("{}", result);
    }

    #[test]
    fn test3() {
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
        let result = Solution::is_valid_sudoku1(board);
        println!("{}", result);
    }

    #[test]
    fn test4() {
        let board = helper1(
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

        let result = Solution::is_valid_sudoku1(board);
        println!("{}", result);
    }

    #[test]
    fn test_block_it() {
        let board = helper1(
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

        let block_it = |i, j| board[i..i + 3].iter().flat_map(move |row| &row[j..j + 3]);

        for i in block_it(0, 0) {
            print!("{i} ");
        }

        // println!("{}", a);

    }

    #[test]
    fn test() {
        let mut a = 2;
        a += 4;
        a %= 3;
        println!("{}", a);
    }

    #[test]
    fn test_iter() {
        let numbers = vec![1, 2, 3, 4, 5];

        let mapped_numbers: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
        println!("Mapped numbers: {:?}", mapped_numbers);

        let flat_mapped_numbers: Vec<i32> = numbers.iter().flat_map(|x| vec![*x, *x]).collect();
        println!("Flat mapped numbers: {:?}", flat_mapped_numbers);

        let filtered_mapped_numbers: Vec<i32> = numbers.iter().filter_map(|x| {
            if x % 2 == 0 {
                Some(x * 3)
            } else {
                None
            }
        }).collect();
        println!("Filtered mapped numbers: {:?}", filtered_mapped_numbers);
    }
}

fn main() {
    println!("Hello, world!");
}

