
fn Ci1(i: i32) -> i32 {
    return i;
}

fn r(N: i32, mut Ci: Vec<i32>) -> i32 {
    Ci.sort();
    let mut total = 1;
    for i in 0..N {
        if let Some(k) = total.checked_mul((Ci[i as usize]-i)) {
            total = k;
        } else {
            println!("overflow!");
            return -1;
        }
    }
    return total;
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use crate::r;


    #[test]
    fn rtest1() {
        let N = 4;
        let Ci = vec![3, 3, 4, 4];
        let result = r(N, Ci);
        println!("{}", result);
    }

    #[test]
    fn rtest2() {
        let N = 4;
        let Ci = vec![4, 3, 2, 1];
        let result = r(N, Ci);
        println!("{}", result);
    }

    #[test]
    fn rtest3() {
        let N = 10;
        let Ci = vec![10, 9, 8, 8, 7, 6, 6, 6, 5, 7];
        let result = r(N, Ci);
        println!("{}", result);
    }
}