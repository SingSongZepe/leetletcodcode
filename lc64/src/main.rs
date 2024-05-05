
struct Solution {
    name: String
}

impl Solution {
    pub fn new(name: String) -> Self {
        Solution {
            name
        }
    }
}

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut minmap = vec![vec![0; grid[0].len()]; grid.len()];
        Solution::min_path(&grid, 0, 0, grid.len() as i32, grid[0].len() as i32, &mut minmap)
    }
    fn min_path(grid: &Vec<Vec<i32>>, r: i32, c:i32, m: i32, n: i32, minmap: &mut Vec<Vec<i32>>) -> i32 {
        if r == m - 1 && c == n - 1 {
            return grid[r as usize][c as usize];
        }
        if r >= m || c >= n {
            return i32::MAX;
        }
        if minmap[r as usize][c as usize] != 0 {
            return minmap[r as usize][c as usize];
        }

        let right = Solution::min_path(grid, r, c + 1, m, n, minmap);
        let down = Solution::min_path(grid, r + 1, c, m, n, minmap);
        let min_path = grid[r as usize][c as usize] + right.min(down);

        minmap[r as usize][c as usize] = min_path;
        min_path
    }
}

struct Solution1;

impl Solution1 {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid[0].len();
        let mut dp = vec![i32::MAX; n];
        dp[0] = 0;

        for r in 0..grid.len()  {
            for c in 0..n {
                dp[c] = grid[r][c] + if c == 0 {
                    dp[c]  // can only come from top
                } else {
                    dp[c-1].min(dp[c])  // came from left or top
                }
            }
        }
        dp[n-1]
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Solution,
        Solution1
    };

    #[test]
    fn test1() {
        let grid = vec![vec![1,3,1],vec![1,5,1],vec![4,2,1]];
        let result = Solution::min_path_sum(grid);
        println!("{:?}", result);
    }

    #[test]
    fn test2() {
        let grid = vec![vec![1,2,3],vec![4,5,6]];
        let result = Solution::min_path_sum(grid);
        println!("{:?}", result);
    }

    #[test]
    fn test11() {
        let grid = vec![vec![1,3,1],vec![1,5,1],vec![4,2,1]];
        let result = Solution1::min_path_sum(grid);
        println!("{:?}", result);
    }

    #[test]
    fn test21() {
        let grid = vec![vec![1,2,3],vec![4,5,6]];
        let result = Solution1::min_path_sum(grid);
        println!("{:?}", result);
    }
}

fn main() {
    println!("Hello, world!");
}
