
pub struct QueensSolver {
    board: Vec<i32>,
    n: usize,
    solutions: i32,
}

impl QueensSolver {
    pub fn new(n: usize) -> Self {
        QueensSolver {
            board: vec![0; n],
            n,
            solutions: 0,
        }
    }

    pub fn solve(&mut self) -> i32 {
        self.backtrack(0);
        self.solutions
    }

    fn backtrack(&mut self, row: usize) {
        if row == self.n {
            self.solutions += 1;
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
}

struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        QueensSolver::new(n as usize).solve()
    }
}

impl Solution {
    pub fn total_n_queens1(n: i32) -> i32 {
        Solution::dfs(n as usize, 0, 0, 0)
    }

    fn dfs(n: usize, r: usize, c: usize, d: usize) -> i32 {
        if r == n { return 1 };
        let mut ans = 0;

        for j in 0..n {
            let (h, v) = (3 * n + r - j, r + j);

            if ((c >> j) & 1) == 0 &&
                ((d >> h) & 1) == 0 &&
                ((d >> v) & 1) == 0 {
                ans += Solution::dfs(
                    n,
                    r + 1,
                    c | (1 << j),
                    d | (1 << h) | (1 << v),
                );
            }
        }
        ans
    }
}



#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let n = 1;
        let result = Solution::total_n_queens(n);
        println!("{result}");
    }

    #[test]
    fn test4() {
        let n = 4;
        let result = Solution::total_n_queens(n);
        println!("{result}");
    }

    #[test]
    fn test8() {
        let n = 8;
        let result = Solution::total_n_queens(n);
        println!("{result}");
    }

    #[test]
    fn test9() {
        let n = 9;
        let result = Solution::total_n_queens(n);
        println!("{result}");
    }

    #[test]
    fn test10() {
        let n = 10;
        let result = Solution::total_n_queens(n);
        println!("{result}");
    }

    #[test]
    fn test12() {
        let n = 12;
        let result = Solution::total_n_queens(n);
        println!("{result}");
    }

    #[test]
    fn test13() {
        let n = 13;
        let result = Solution::total_n_queens(n);
        println!("{result}");
    }

    #[test]
    fn test14() {
        let n = 14;
        let result = Solution::total_n_queens(n);
        println!("{result}");
    }

    #[test]
    fn test131() {
        let n = 13;
        let result = Solution::total_n_queens1(n);
        println!("{result}");
    }

    #[test]
    fn test141() {
        let n = 14;
        let result = Solution::total_n_queens1(n);
        println!("{result}");
    }

    #[test]
    fn test151() {
        let n = 15;
        let result = Solution::total_n_queens1(n);
        println!("{result}");
    }
}

fn main() {
    println!("Hello, world!");
}
