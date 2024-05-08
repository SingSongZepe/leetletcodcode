
struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.len() == 1 {
            return Solution::find_col(&matrix, 0, 0, matrix[0].len()-1, target).is_some();
        }
        match Solution::find_row(&matrix, 0, matrix.len()-1, target) {
            Some(from) => match Solution::find_col(&matrix, from, 0, matrix[0].len()-1, target) {
                Some(_) => true,
                None => false,
            },
            None => false,
        }
    }
    pub fn find_row(matrix: &Vec<Vec<i32>>, from: usize, to: usize, target: i32) -> Option<usize> {
        if from > to {
            return None;
        }
        let mid = from + (to - from) / 2;
        if matrix[mid][0] <= target && (mid == to || matrix[mid + 1][0] > target) {
            Some(mid)
        } else if matrix[mid][0] > target {
            if mid >= 1  {
                Solution::find_row(matrix, from, mid - 1, target)
            } else {
                None
            }
        } else {
            if mid + 1 < matrix.len() {
                Solution::find_row(matrix, mid + 1, to, target)
            } else {
                None
            }
        }
    }
    pub fn find_col(matrix: &Vec<Vec<i32>>, row: usize, from: usize, to: usize, target: i32) -> Option<()> {
        if from > to {
            return None;
        }
        let mid = from + (to - from) / 2;
        if matrix[row][mid] == target {
            Some(())
        } else if matrix[row][mid] < target {
            if mid + 1 < matrix[row].len() {
                Solution::find_col(matrix, row, mid + 1, to, target)
            } else {
                None
            }
        } else {
            if mid >= 1 {
                Solution::find_col(matrix, row, from, mid - 1, target)
            } else {
                None
            }
        }
    }
}


struct Solution1;

impl Solution1 {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut r = 0 as i32;
        let mut c = cols as i32 - 1;
        while r < rows as i32 && c as i32 >= 0 {
            let curr = matrix[r as usize][c as usize];
            if curr == target {
                return true;
            } else if curr < target {
                r += 1;
            } else {
                c -= 1;
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
    };

    #[test]
    fn test1() {
        let matrix = vec![
            vec![1, 3, 5, 7],
            vec![10, 11, 16, 20],
            vec![23, 30, 34, 60]
        ];
        let target = 3;
        let result = Solution::search_matrix(matrix, target);
        println!("{:?}", result);
    }

    #[test]
    fn test2() {
        let matrix = vec![
            vec![1, 3, 5, 7],
            vec![10, 11, 16, 20],
            vec![23, 30, 34, 60]
        ];
        let target = 29;
        let result = Solution::search_matrix(matrix, target);
        println!("{:?}", result);
    }

    #[test]
    fn test3() {
        let matrix = vec![
            vec![1],
        ];
        let target = 0;
        let result = Solution::search_matrix(matrix, target);
        println!("{:?}", result);
    }

    #[test]
    fn test4() {
        let matrix = vec![
            vec![1],
        ];
        let target = 1;
        let result = Solution::search_matrix(matrix, target);
        println!("{:?}", result);
    }

    #[test]
    fn test5() {
        let matrix = vec![
            vec![1],
            vec![3],
        ];
        let target = 0;
        let result = Solution::search_matrix(matrix, target);
        println!("{:?}", result);
    }

    #[test]
    fn test6() {
        let matrix = vec![
            vec![1],
            vec![3],
        ];
        let target = 3;
        let result = Solution::search_matrix(matrix, target);
        println!("{:?}", result);
    }

    #[test]
    fn test11() {
        let matrix = vec![
            vec![1, 3, 5, 7],
            vec![10, 11, 16, 20],
            vec![23, 30, 34, 60]
        ];
        let target = 3;
        let result = Solution1::search_matrix(matrix, target);
        println!("{:?}", result);
    }

    #[test]
    fn test21() {
        let matrix = vec![
            vec![1, 3, 5, 7],
            vec![10, 11, 16, 20],
            vec![23, 30, 34, 60]
        ];
        let target = 29;
        let result = Solution1::search_matrix(matrix, target);
        println!("{:?}", result);
    }

    #[test]
    fn test31() {
        let matrix = vec![
            vec![1],
        ];
        let target = 0;
        let result = Solution1::search_matrix(matrix, target);
        println!("{:?}", result);
    }

    #[test]
    fn test41() {
        let matrix = vec![
            vec![1],
        ];
        let target = 1;
        let result = Solution1::search_matrix(matrix, target);
        println!("{:?}", result);
    }

    #[test]
    fn test51() {
        let matrix = vec![
            vec![1],
            vec![3],
        ];
        let target = 0;
        let result = Solution1::search_matrix(matrix, target);
        println!("{:?}", result);
    }

    #[test]
    fn test61() {
        let matrix = vec![
            vec![1],
            vec![3],
        ];
        let target = 3;
        let result = Solution1::search_matrix(matrix, target);
        println!("{:?}", result);
    }



    #[test]
    fn test_find_row1() {
        let matrix = vec![
            vec![1, 3, 5, 7],
            vec![10, 11, 16, 20],
            vec![23, 30, 34, 60]
        ];
        let target = 13;
        let result = Solution::find_row(&matrix, 0, matrix.len()-1, target).unwrap();
        println!("{}", result);
    }

    #[test]
    fn test_option() {
        let x = Option::from(());
        match x {
            Some(_) => println!("Some"),
            None => println!("None"),
        }
        // let y = () as Option<()>;
        // match y {
        //     Some(_) => println!("Some"),
        //     None => println!("None"),
        // }

        fn return_some() -> Option<()> {
            Some(())
        }
        fn return_none() -> Option<()> {
            None
        }

        match return_some() {
            Some(_) => println!("Some"),
            None => println!("None"),
        }
        match return_none() {
            Some(_) => println!("Some"),
            None => println!("None"),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
