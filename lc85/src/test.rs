

#[cfg(test)]
mod tests {
    use crate::{
        Solution,
        helper,
    };

    #[test]
    fn test1() {
        // matrix to vec [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
        let matrix = helper(vec![vec!["1","0","1","0","0"],vec!["1","0","1","1","1"],vec!["1","1","1","1","1"],vec!["1","0","0","1","0"]]);
        let result = Solution::maximal_rectangle(matrix);
        println!("{result:?}");
    }


    #[test]
    fn test2() {
        // matrix to vec [[0]]
        let matrix = helper(vec![vec!["0"]]);
        let result = Solution::maximal_rectangle(matrix);
        println!("{result:?}");
    }

    #[test]
    fn test3() {
        // matrix to vec [[0]]
        let matrix = helper(vec![vec!["1"]]);
        let result = Solution::maximal_rectangle(matrix);
        println!("{result:?}");
    }
}