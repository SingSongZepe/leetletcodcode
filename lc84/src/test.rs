

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
fn test1() {
    let heights = vec![2, 1, 5, 6, 2, 3];
    let result = Solution::largest_rectangle_area(heights);
    println!("{result}"); // expected output: 10
}

    #[test]
    fn test2() {
        let heights = vec![2, 4];
        let result = Solution::largest_rectangle_area(heights);
        println!("{result}"); // expected output: 4
    }

    #[test]
    fn test3() {
        let heights = vec![1,1,1,1,1,1,1,1,1,1,1,1,2, 4];
        let result = Solution::largest_rectangle_area(heights);
        println!("{result}"); // expected output: 4
    }

    #[test]
    fn test4() {
        let heights = vec![2,5,6];
        let result = Solution::largest_rectangle_area(heights);
        println!("{result}"); // expected output: 4
    }

    #[test]
    fn test11() {
        let heights = vec![2, 1, 5, 6, 2, 3];
        let result = Solution1::largest_rectangle_area(heights);
        println!("{result}"); // expected output: 10
    }

    #[test]
    fn test21() {
        let heights = vec![2, 4];
        let result = Solution1::largest_rectangle_area(heights);
        println!("{result}"); // expected output: 4
    }
}