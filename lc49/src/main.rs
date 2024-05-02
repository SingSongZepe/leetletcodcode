
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hm = HashMap::<Vec<i32>, Vec<usize>>::new();
        for (idx, str) in strs.iter().enumerate() {
            let mut v = vec![0; 26];
            for c in str.chars() {
                v[c as usize - 97] += 1;
            }
            hm.entry(v).or_insert_with(Vec::new).push(idx);
        }
        let mut res = vec![];
        for (_, value) in &hm {
            res.push(value.iter().map(|i| strs[*i].to_owned()).collect());
        }
        res
    }

    pub fn group_anagrams1(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hm = HashMap::<Vec<i32>, usize>::new();
        let mut res: Vec<Vec<String>> = vec![];
        for str in strs {
            let mut v = vec![0; 26];
            for c in str.chars() {
                v[c as usize - 97] += 1;
            }
            match hm.get(&v) {
                Some(u) => {
                    res[*u].push(str.to_owned());
                }
                None => {
                    res.push(vec![str.to_owned()]);
                    hm.insert(v, res.len()-1);
                }
            }
        }
        res
    }
}

impl Solution {
    pub fn group_anagrams_by_prime(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hm = HashMap::<i32, usize>::new();
        let mut res: Vec<Vec<String>> = vec![];
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101];
        for str in strs {
            let mut sum = 1;
            for c in str.chars() {
                sum *= primes[c as usize - 97];
            }
            match hm.get(&sum) {
                Some(u) => {
                    res[*u].push(str.to_owned());
                }
                None => {
                    res.push(vec![str.to_owned()]);
                    hm.insert(sum, res.len()-1);
                }
            }
        }
        res
    }
}

use std::borrow::Cow;

impl Solution {
    pub fn group_anagrams3(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut group_map: HashMap<String, Vec<String>> = HashMap::new();
        for word in strs.into_iter().map(Cow::from) {
            let mut sorted = word.to_string();
            unsafe { sorted.as_mut_vec().sort() }
            group_map
                .entry(sorted)
                .and_modify(|v| v.push(word.to_string()))
                .or_insert(vec![word.to_string()]);
        }
        group_map.into_values().collect()
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::Solution;

    #[test]
    fn test1() {
        let strs = ["eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string()].to_vec();
        let result = Solution::group_anagrams(strs);
        println!("{result:?}");
    }

    #[test]
    fn test2() {
        let strs = ["".to_string()].to_vec();
        let result = Solution::group_anagrams(strs);
        println!("{result:?}");
    }

    #[test]
    fn test3() {
        let strs = ["a".to_string()].to_vec();
        let result = Solution::group_anagrams(strs);
        println!("{result:?}");
    }

    #[test]
    fn test11() {
        let strs = ["eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string()].to_vec();
        let result = Solution::group_anagrams1(strs);
        println!("{result:?}");
    }

    #[test]
    fn test21() {
        let strs = ["".to_string()].to_vec();
        let result = Solution::group_anagrams1(strs);
        println!("{result:?}");
    }

    #[test]
    fn test31() {
        let strs = ["a".to_string()].to_vec();
        let result = Solution::group_anagrams1(strs);
        println!("{result:?}");
    }

    #[test]
    fn test12() {
        let strs = ["eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string()].to_vec();
        let result = Solution::group_anagrams_by_prime(strs);
        println!("{result:?}");
    }

    #[test]
    fn test22() {
        let strs = ["".to_string()].to_vec();
        let result = Solution::group_anagrams_by_prime(strs);
        println!("{result:?}");
    }

    #[test]
    fn test32() {
        let strs = ["a".to_string()].to_vec();
        let result = Solution::group_anagrams_by_prime(strs);
        println!("{result:?}");
    }

    #[test]
    fn test_hashmap_vec_bool_usize() {
        let mut hm = HashMap::<Vec<bool>, usize>::new();
        let mut v = vec![false; 26];
        hm.insert(v.clone(), 1);
        v[4] = true;
        hm.insert(v.clone(), 5);
        let v2 = vec![false; 26];
        let mut v3 = vec![false; 26];
        v3[4] = true;
        println!("{}", hm.get::<Vec<bool>>(&v2).unwrap());
        println!("{}", hm.get::<Vec<bool>>(&v3).unwrap());
    }

    #[test]
    fn test_hashmap_vec_i32_usize() {
        let mut hm = HashMap::<Vec<i32>, usize>::new();
        let mut v = vec![0; 26];
        hm.insert(v.clone(), 1);
        v[4] = 1; // e 1
        hm.insert(v.clone(), 5);
        let v2 = vec![0; 26];
        let mut v3 = vec![0; 26];
        v3[4] = 1;
        println!("{}", hm.get::<Vec<i32>>(&v2).unwrap());
        println!("{}", hm.get::<Vec<i32>>(&v3).unwrap());
    }

    #[test]
    fn test_hashmap_vec_i32_vec_usize() {
        let mut hm = std::collections::HashMap::<Vec<i32>, Vec<usize>>::new();
        let mut strs = vec![vec![0; 26], vec![0; 26], vec![0; 26], vec![0; 26], vec![0; 26]];
        strs[0][5] = 1; // f 1
        strs[1][4] = 8; // e 8
        strs[2][5] = 1; // f 1
        strs[3][5] = 1;
        strs[4][5] = 1;
        for (idx, str) in strs.iter().enumerate() {
            hm.entry(str.clone()).or_insert_with(Vec::new).push(idx);
        }
        let mut nstrs = vec![vec![0; 26], vec![0; 26]];
        nstrs[0][5] = 1; // f 1
        nstrs[1][4] = 8; // e 8
        println!("{:?}", hm.get(&nstrs[0]).unwrap());
        println!("{:?}", hm.get(&nstrs[1]).unwrap());
    }
}

fn main() {
    println!("Hello, world!");
}
