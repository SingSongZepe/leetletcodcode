
#[cfg(test)]
mod tests {
    use crate::{
        Solution,
        Solution1,
        Solution2,
        helper,
        print_list,
    };

    #[test]
    fn test1() {
        let v = vec![1, 2, 3, 3, 4, 4, 5];
        let head = helper(v);
        let result = Solution::delete_duplicates(head);
        print_list(&result); // expected [1, 2, 5]
    }

    #[test]
    fn test2() {
        let v = vec![1, 1, 1, 2, 3];
        let head = helper(v);
        let result = Solution::delete_duplicates(head);
        print_list(&result);
    }

    #[test]
    fn test11() {
        let v = vec![1, 2, 3, 3, 4, 4, 5];
        let head = helper(v);
        let result = Solution1::delete_duplicates(head);
        print_list(&result);
    }

    #[test]
    fn test21() {
        let v = vec![1, 1, 1, 2, 3];
        let head = helper(v);
        let result = Solution1::delete_duplicates(head);
        print_list(&result);
    }

    #[test]
    fn test31() {
        let v = vec![];
        let head = helper(v);
        let result = Solution1::delete_duplicates(head);
        print_list(&result);
    }

    #[test]
    fn test12() {
        let v = vec![1, 2, 3, 3, 4, 4, 5];
        let head = helper(v);
        let result = Solution2::delete_duplicates(head);
        print_list(&result);
    }

    #[test]
    fn test22() {
        let v = vec![1, 1, 1, 2, 3];
        let head = helper(v);
        let result = Solution2::delete_duplicates(head);
        print_list(&result);
    }

    #[test]
    fn test32() {
        let v = vec![];
        let head = helper(v);
        let result = Solution2::delete_duplicates(head);
        print_list(&result);
    }

    #[test]
    fn test42() {
        let v = vec![1, 1, 1, 2, 3, 4, 4, 4,4,4,4,5,6,6,7,8,8,8,8,8,9];
        let head = helper(v);
        let result = Solution2::delete_duplicates(head);
        print_list(&result);
    }
}
