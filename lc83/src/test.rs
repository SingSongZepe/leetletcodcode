
#[cfg(test)]
mod tests {
    use crate::{
        Solution,
        Solution1,
        Solution2,
        Solution3,
        helper,
        print_list,
    };

    #[test]
    fn test1() {
        let head = helper(vec![1, 1, 2]);
        let result = Solution::delete_duplicates(head);
        print_list(&result);
    }

    #[test]
    fn test2() {
        let head = helper(vec![1,1,2,3,3]);
        let result = Solution::delete_duplicates(head);
        print_list(&result);
    }

    #[test]
    fn test3() {
        let head = helper(vec![1,1,2,3,3,3, 4,5,6,6,7,7,7,7,8,8,9,9,10, 13]);
        let result = Solution::delete_duplicates(head);
        print_list(&result);
    }

    #[test]
    fn test4() {
        let head = helper(vec![1]);
        let result = Solution::delete_duplicates(head);
        print_list(&result);
    }

    #[test]
    fn test11() {
        let head = helper(vec![1, 1, 2]);
        let result = Solution1::delete_duplicates(head);
        print_list(&result);
    }

    #[test]
    fn test21() {
        let head = helper(vec![1,1,2,3,3]);
        let result = Solution1::delete_duplicates(head);
        print_list(&result);
    }

    #[test]
    fn test31() {
        let head = helper(vec![1,1,2,3,3,3, 4,5,6,6,7,7,7,7,8,8,9,9,10, 13]);
        let result = Solution1::delete_duplicates(head);
        print_list(&result);
    }

    #[test]
    fn test41() {
        let head = helper(vec![1]);
        let result = Solution1::delete_duplicates(head);
        print_list(&result);
    }

    #[test]
    fn test51() {
        let head = helper(vec![]);
        let result = Solution1::delete_duplicates(head);
        print_list(&result);
    }

    #[test]
    fn test61() {
        let head = helper(vec![0,0,0,0,0]);
        let result = Solution1::delete_duplicates(head);
        print_list(&result);
    }



    // Solution2 tests
    #[test]
    fn test12() {
        let head = helper(vec![1, 1, 2]);
        let result = Solution2::delete_duplicates(head);
        print_list(&result);
    }

    #[test]
    fn test22() {
        let head = helper(vec![1,1,2,3,3]);
        let result = Solution2::delete_duplicates(head);
        print_list(&result);
    }

    #[test]
    fn test32() {
        let head = helper(vec![1,1,2,3,3,3, 4,5,6,6,7,7,7,7,8,8,9,9,10, 13]);
        let result = Solution2::delete_duplicates(head);
        print_list(&result);
    }

    #[test]
    fn test42() {
        let head = helper(vec![1]);
        let result = Solution2::delete_duplicates(head);
        print_list(&result);
    }

    #[test]
    fn test52() {
        let head = helper(vec![]);
        let result = Solution2::delete_duplicates(head);
        print_list(&result);
    }

    // Solution3 tests
    #[test]
    fn test13() {
        let head = helper(vec![1, 1, 2]);
        let result = Solution3::delete_duplicates(head);
        print_list(&result);
    }

    #[test]
    fn test23() {
        let head = helper(vec![1,1,2,3,3]);
        let result = Solution3::delete_duplicates(head);
        print_list(&result);
    }

    #[test]
    fn test33() {
        let head = helper(vec![1,1,2,3,3,3, 4,5,6,6,7,7,7,7,8,8,9,9,10, 13]);
        let result = Solution3::delete_duplicates(head);
        print_list(&result);
    }

    #[test]
    fn test43() {
        let head = helper(vec![1]);
        let result = Solution3::delete_duplicates(head);
        print_list(&result);
    }

    #[test]
    fn test53() {
        let head = helper(vec![]);
        let result = Solution3::delete_duplicates(head);
        print_list(&result);
    }
}