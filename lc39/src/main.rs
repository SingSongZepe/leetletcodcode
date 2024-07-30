
struct Solution;

impl Solution {

    // 100% at speed  not bad
    // 68% at memory maybe okay
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        Solution::combine(&candidates, target, &mut vec![], &mut result, 0);
        result
    }

    pub fn combine(candidates: &Vec<i32>, target: i32, select: &mut Vec<i32>, result: &mut Vec<Vec<i32>>, from: usize) {
        for idx in from..candidates.len() {
            let i = candidates[idx];
            if target == i {
                select.push(i);
                result.push(select.clone());
                select.pop(); // !
            } else if target - i >= 0 {
                select.push(i);
                Solution::combine(candidates, target - i, select, result, idx);
                select.pop(); // !
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let result = Solution::combination_sum(candidates, target);
        println!("{:?}", result); // excepted [[2,2,3],[7]]
    }


    #[test]
    fn test2() {
        let candidates = vec![2, 3, 5];
        let target = 8;
        let result = Solution::combination_sum(candidates, target);
        println!("{:?}", result); // excepted [[2,2,2,2],[2,3,3],[3,5]]
    }

    #[test]
    fn test3() {
        let candidates = vec![2];
        let target = 1;
        let result = Solution::combination_sum(candidates, target);
        println!("{:?}", result); // excepted []
    }

    #[test]
    fn test4() {
        let mut select = vec![1,2,3,4];
        println!("{:?}", select);
        select.pop();
        println!("{:?}", select);
    }
}

fn main() {
    println!("Hello, world!");
}
