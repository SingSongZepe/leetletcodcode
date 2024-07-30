
struct Solution;

use std::collections::HashMap;
impl Solution {
    // hash map
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut hm = HashMap::<u8, Vec<(usize, usize)>>::new();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                hm.entry(board[i][j] as u8).or_insert(vec![]).push((i, j));
            }
        }
        let mut visited = vec![vec![false; board[0].len()]; board.len()];
        drop(board);
        let word = word.as_bytes();

        // entry
        let mut positions = Vec::new();
        if let Some(poss) = hm.get(&word[0]) {
            positions.extend(poss.clone());
        } else {
            return false;
        }
        for pos in positions{
            visited[pos.0][pos.1] = true;
            if Solution::find(&mut hm, word, &mut visited, 1, pos) {
                return true;
            }
            visited[pos.0][pos.1] = false;
        }
        false
    }
    // start with index 1
    fn find(hm: &mut HashMap<u8, Vec<(usize, usize)>>, word: &[u8], visited: &mut Vec<Vec<bool>>, from: usize, pos: (usize, usize)) -> bool {
        // macro_rules! pos {
        //     ($i:tt) => {
        //         pos.$i
        //     };
        // }
        if from == word.len() {
            return true;
        }
        let mut positions = Vec::new();
        if let Some(poss) = hm.get(&word[from]) {
            positions.extend(poss.clone());
        } else {
            return false;
        }
        let mut directions = Vec::with_capacity(4);
        // left
        if pos.0 > 0 {
            directions.push((pos.0-1, pos.1));
        }
        // right
        if pos.0 < visited.len()-1 {
            directions.push((pos.0+1, pos.1));
        }
        // up
        if pos.1 > 0 {
            directions.push((pos.0, pos.1-1));
        }
        // down
        if pos.1 < visited[0].len()-1 {
            directions.push((pos.0, pos.1+1));
        }
        for p in directions {
            if !visited[p.0][p.1] && positions.contains(&p) {
                visited[p.0][p.1] = true;
                if Solution::find(hm, word, visited, from+1, p) {
                    return  true;
                }
                visited[p.0][p.1] = false;
            }
        }
        false
    }
}


struct Solution1;

impl Solution1 {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let n = board.len();
        let m = board[0].len();
        let w = word.chars().collect::<Vec<char>>();
        let mut visited = vec![vec![false; m]; n];
        for i in 1..=n {
            for j in 1..=m {
                if board[i - 1][j - 1] == w[0] {
                    if Self::rummage(&board, &w, &mut visited, i, j, 0) {
                        return true;
                    }
                }
            }
        }
        false
    }
    fn rummage(board    : &[Vec<char>],
               word     : &[char],
               visited  : &mut [Vec<bool>],
               i        : usize,
               j        : usize,
               k        : usize)
               -> bool
    {
        if k == word.len() {
            true
        }
        else if i < 1 || i > board.len() ||
            j < 1 || j > board[0].len() ||
            visited[i - 1][j - 1] || board[i - 1][j - 1] != word[k]
        {
            false
        }
        else {
            visited[i - 1][j - 1] = true;
            let res = Self::rummage(board, word, visited, i + 1, j, k + 1) ||
                Self::rummage(board, word, visited, i - 1, j, k + 1) ||
                Self::rummage(board, word, visited, i, j + 1, k + 1) ||
                Self::rummage(board, word, visited, i, j - 1, k + 1);
            visited[i - 1][j - 1] = false;
            res
        }
    }
}

struct Solution2;

impl Solution2 {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        fn solve(board: &mut Vec<Vec<char>>, i: usize, j: usize, word: &str) -> bool {
            if word.is_empty() {return true;}
            let tmp = board[i][j];
            board[i][j] = '#';
            if i>0 && word.chars().next().unwrap() == board[i-1][j] {
                if solve(board, i-1, j, &word[1..]) {return true;}
            }
            if i<board.len()-1 && word.chars().next().unwrap() == board[i+1][j] {
                if solve(board, i+1, j, &word[1..]) {return true;}
            }
            if j>0 && word.chars().next().unwrap() == board[i][j-1] {
                if solve(board, i, j-1, &word[1..]) {return true;}
            }
            if j<board[0].len()-1 && word.chars().next().unwrap() == board[i][j+1] {
                if solve(board, i, j+1, &word[1..]) {return true;}
            }
            board[i][j] = tmp;
            false
        }

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == word.chars().next().unwrap() &&
                    solve(&mut board, i, j, &word[1..]) {return true;}
            }
        }
        false
    }
}

struct Solution3;

impl Solution3 {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut hm = HashMap::<u8, Vec<(usize, usize)>>::new();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                hm.entry(board[i][j] as u8).or_insert(vec![]).push((i, j));
            }
        }
        let mut visited = vec![vec![false; board[0].len()]; board.len()];
        let word = word.as_bytes();

        let mut positions: Vec<(usize,usize)> = Vec::new();
        if let Some(poss) = hm.get(&word[0]) {
            positions.extend(poss);
        } else {
            return false;
        }
        for pos in positions {
            visited[pos.0][pos.1] = true;
            if Self::find(&mut hm, word, &mut visited, 1, pos) {
                return true;
            }
            visited[pos.0][pos.1] = false;
        }
        false
    }

    fn find(hm: &HashMap<u8, Vec<(usize, usize)>>, word: &[u8], visited: &mut Vec<Vec<bool>>, from: usize, pos: (usize, usize)) -> bool {
        if from == word.len() {
            return true;
        }

        let binding = Vec::new();
        let mut positions = hm.get(&word[from]).unwrap_or(&binding);

        let mut directions = Vec::with_capacity(4);
        let (row, col) = (visited.len(), visited[0].len());
        if pos.0 > 0 {
            directions.push((pos.0 - 1, pos.1));
        }
        if pos.0 + 1 < row {
            directions.push((pos.0 + 1, pos.1));
        }
        if pos.1 > 0 {
            directions.push((pos.0, pos.1 - 1));
        }
        if pos.1 + 1 < col {
            directions.push((pos.0, pos.1 + 1));
        }

        for p in directions {
            if !visited[p.0][p.1] && positions.contains(&p) {
                visited[p.0][p.1] = true;
                if Self::find(hm, word, visited, from + 1, p) {
                    return true;
                }
                visited[p.0][p.1] = false;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Solution,
        Solution1,
        Solution2,
        Solution3,
    };

    #[test]
    fn test1() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E']
        ];
        let word = "ABCCED".to_string();
        let result = Solution::exist(board, word);
        println!("{}", result);
    }

    #[test]
    fn test2() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E']
        ];
        let word = "SEE".to_string();
        let result = Solution::exist(board, word);
        println!("{}", result);
    }

    #[test]
    fn test3() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E']
        ];
        let word = "ABCB".to_string();
        let result = Solution::exist(board, word);
        println!("{}", result);
    }

    #[test]
    fn test4() {
        let board = vec![vec!['a']];
        let word = "ab".to_string();
        let result = Solution::exist(board, word);
        println!("{}", result);
    }

    #[test]
    fn test5() {
        let board = vec![vec!['a']];
        let word = "b".to_string();
        let result = Solution::exist(board, word);
        println!("{}", result);
    }

    #[test]
    fn test6() {
        let board = vec![vec!['a', 'a']];
        let word = "aaa".to_string();
        let result = Solution::exist(board, word);
        println!("{}", result);
    }

    #[test]
    fn test13() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E']
        ];
        let word = "ABCCED".to_string();
        let result = Solution3::exist(board, word);
        println!("{}", result);
    }

    #[test]
    fn test23() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E']
        ];
        let word = "SEE".to_string();
        let result = Solution3::exist(board, word);
        println!("{}", result);
    }

    #[test]
    fn test33() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E']
        ];
        let word = "ABCB".to_string();
        let result = Solution3::exist(board, word);
        println!("{}", result);
    }

    #[test]
    fn test43() {
        let board = vec![vec!['a']];
        let word = "ab".to_string();
        let result = Solution3::exist(board, word);
        println!("{}", result);
    }

    #[test]
    fn test53() {
        let board = vec![vec!['a']];
        let word = "b".to_string();
        let result = Solution3::exist(board, word);
        println!("{}", result);
    }

    #[test]
    fn test63() {
        let board = vec![vec!['a', 'a']];
        let word = "aaa".to_string();
        let result = Solution3::exist(board, word);
        println!("{}", result);
    }

    #[test]
    fn test_marco() {
        let pos = (1, 2);
        macro_rules! pos {
            ($index:tt) => {
                pos.$index
            };
        }
        println!("{}", pos!(0));
    }
}

fn main() {
    println!("Hello, world!");
}
