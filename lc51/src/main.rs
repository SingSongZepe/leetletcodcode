use std::fmt::Display;

pub struct QueensSolver {
    board: Vec<i32>,
    n: usize,
    solutions: Vec<Vec<String>>,
}

impl QueensSolver {
    pub fn new(n: usize) -> Self {
        QueensSolver {
            board: vec![0; n],
            n,
            solutions: Vec::new(),
        }
    }

    pub fn solve(&mut self) -> Vec<Vec<String>> {
        self.backtrack(0);
        self.solutions.to_owned()
    }

    fn backtrack(&mut self, row: usize) {
        if row == self.n {
            self.solutions.push(self.generate_solution());
            return;
        }

        for col in 0..self.n {
            if self.is_valid_placement(row, col) {
                self.board[row] = col as i32;
                self.backtrack(row + 1);
                self.board[row] = 0;
            }
        }
    }

    fn is_valid_placement(&self, row: usize, col: usize) -> bool {
        for i in 0..row {
            if self.board[i] == col as i32 || (row as i32 - i as i32).abs() == (col as i32 - self.board[i]).abs() {
                return false;
            }
        }
        true
    }

    fn generate_solution(&self) -> Vec<String> {
        let mut solution = Vec::new();
        for &col in &self.board {
            let mut row_chars: Vec<char> = vec!['.'; self.n];
            row_chars[col as usize] = 'Q';
            solution.push(row_chars.into_iter().collect());
        }
        solution
    }
}

struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        // 0 empty
        // > 0 can't place // maybe queen
        let mut board = vec![vec![0; n as usize]; n as usize];
        let mut result = vec![];
        let mut queue: Vec<(usize, usize)> = Vec::with_capacity(n as usize); // (r, c)
        Solution::place(&mut board, n, &mut result, &mut queue);
        result
    }

    pub fn place(board: &mut Vec<Vec<i32>>, remain: i32, result: &mut Vec<Vec<String>>, queue: &mut Vec<(usize, usize)>) { // remain
        if remain == 0 {
            result.push(Solution::turn_view(queue, board.len() as i32));
            return;
        }
        let len = board.len();
        for r in 0..len {
            for c in 0..len {
                if board[r][c] == 0 {
                    Solution::add_q(board, r, c);
                    if Solution::count_empty(board) < remain - 1 {
                        Solution::remove_q(board, r, c);
                        continue
                    }
                    // if not suitable, continue
                    queue.push((r, c));
                    Solution::place(board, remain-1, result, queue);
                    Solution::remove_q(board, r, c);
                    queue.pop();
                }
            }
        }
    }

    pub fn add_q(board: &mut Vec<Vec<i32>>, r: usize, c: usize) {
        if board[r][c] != 0 {
            return;
        }
        let n = board.len();
        for i in 0..n {
            board[r][i] += 1;
            board[i][c] += 1;
        }
        board[r][c] -= 1;
        let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
        for &(dr, dc) in &directions {
            let mut i = r as i32 + dr;
            let mut j = c as i32 + dc;

            while i >= 0 && i < n as i32 && j >= 0 && j < n as i32 {
                board[i as usize][j as usize] += 1;
                i += dr;
                j += dc;
            }
        }
    }

    pub fn remove_q(board: &mut Vec<Vec<i32>>, r: usize, c: usize) {
        if board[r][c] == 0 {
            return;
        }
        let n = board.len();
        for i in 0..n {
            board[r][i] -= 1;
            board[i][c] -= 1;
        }
        board[r][c] += 1;
        let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
        for &(dr, dc) in &directions {
            let mut i = r as i32 + dr;
            let mut j = c as i32 + dc;

            while i >= 0 && i < n as i32 && j >= 0 && j < n as i32 {
                board[i as usize][j as usize] -= 1;
                i += dr;
                j += dc;
            }
        }
    }

    pub fn turn_view(queue: &Vec<(usize, usize)>, n: i32) -> Vec<String> {
        let mut view = vec![String::from(""); n as usize];
        for &(r, c) in queue {
            let mut row_chars: Vec<char> = vec!['.'; n as usize];
            row_chars[c as usize] = 'Q';
            view[r as usize] = row_chars.iter().collect();
        }
        for i in 0..n as usize {
            if view[i].is_empty() {
                view[i] = ".".repeat(n as usize);
            }
        }
        view
    }

    pub fn count_empty(board: &Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        for row in board {
            for i in row {
                if *i == 0 {
                    count += 1;
                }
            }
        }
        count
    }
}

impl Solution {
    pub fn solve_n_queens1(n: i32) -> Vec<Vec<String>> {
        QueensSolver::new(n as usize).solve()
    }
}

pub fn print_board<T>(board: &Vec<Vec<T>>)
    where
        T: Display + std::fmt::Debug
{
    for line in board {
        println!("{line:?}");
    }
    println!();
}

pub fn print_view<T>(board: &Vec<T>)
    where
        T: Display + std::fmt::Debug
{
    for str in board {
        println!("{}", str);
    }
    println!();
}



#[cfg(test)]
mod tests {
    use crate::{print_board, print_view, Solution};

    // n [1, 9]
    #[test]
    fn test1() {
        let n = 4;
        let result = Solution::solve_n_queens(n);
        println!("{:?}", result);
    }

    #[test]
    fn test2() {
        let n = 1;
        let result = Solution::solve_n_queens(n);
        println!("{:?}", result);
    }

    #[test]
    fn test3() {
        let n = 8;
        let result = Solution::solve_n_queens(n);
        println!("{:?}", result);
    }

    #[test]
    fn test4() {
        let n = 7;
        let result = Solution::solve_n_queens(n);
        println!("{:?}", result);
    }

    #[test]
    fn test5() {
        let n = 6;
        let result = Solution::solve_n_queens(n);
        println!("{:?}", result);
    }

    #[test]
    fn test6() {
        let n = 5;
        let result = Solution::solve_n_queens(n);
        println!("{:?}", result);
    }

    #[test]
    fn test11() {
        let n = 1;
        let result = Solution::solve_n_queens1(n);
        println!("{:?}", result);
    }

    #[test]
    fn test41() {
        let n = 4;
        let result = Solution::solve_n_queens1(n);
        println!("{:?}", result);
    }

    #[test]
    fn test61() {
        let n = 5;
        let result = Solution::solve_n_queens1(n);
        println!("{:?}", result);
    }

    #[test]
    fn test91() {
        let n = 9;
        let result = Solution::solve_n_queens1(n);
        println!("{:?}", result);
    }

    #[test]
    fn test301() {
        let n = 30;
        let result = Solution::solve_n_queens1(n);
        println!("{:?}", result);
    }

    #[test]
    fn test_add_q1() {
        let n = 7;
        let mut board = vec![vec![0; n as usize]; n as usize];
        Solution::add_q(&mut board, 2, 3);
        print_board(&board);
    }

    #[test]
    fn test_add_q2() {
        let n = 7;
        let mut board = vec![vec![0; n as usize]; n as usize];
        Solution::add_q(&mut board, 0, 0);
        print_board(&board);
    }

    #[test]
    fn test_remove_q1() {
        let n = 7;
        let mut board = vec![vec![0; n as usize]; n as usize];
        Solution::add_q(&mut board, 2, 3);
        print_board(&board);
        Solution::remove_q(&mut board, 2, 3);
        print_board(&board);
    }

    #[test]
    fn test_remove_q2() {
        let n = 7;
        let mut board = vec![vec![0; n as usize]; n as usize];
        Solution::add_q(&mut board, 0, 0);
        print_board(&board);
        Solution::remove_q(&mut board, 0, 0);
        print_board(&board);
    }

    #[test]
    fn test_turn_view() {
        let queue = vec![(0, 0), (3, 0)];
        let view = Solution::turn_view(&queue, 9);
        print_view(&view);
    }
}

fn main() {
    println!("Hello, world!");
}
