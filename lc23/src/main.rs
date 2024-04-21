// 1. to implement a PRIORITY QUEUE, the BINARY HEAP data structure is chosen
use std::collections::BinaryHeap;

// 2. entire Box<ListNode> is stored in the BINARY HEAP, not just ListNode.val;
//    this requires implementation of Ord and PartialOrd for ListNode
use std::cmp::Ordering;

impl PartialOrd<ListNode> for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
        other.val.partial_cmp(&self.val)     // - for MIN HEAP
        //self.val.partial_cmp(&other.val)   // - for MAX HEAP
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)             // - for MIN HEAP
        //self.val.cmp(&other.val)           // - for MAX HEAP
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

fn helper(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut current = head.as_mut().unwrap();

    for val in nums {
        current.next = Some(Box::new(ListNode::new(val)));
        current = current.next.as_mut().unwrap();
    }

    head.unwrap().next
}

fn print_list(head: &Option<Box<ListNode>>) {
    let mut current = head.as_ref();
    while let Some(node) = current {
        print!("{} ", node.val);
        current = current.unwrap().next.as_ref();
    }
    println!("");
}

struct Solution;

impl Solution {
    // just beats 11% at speed, 55% at memory
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }
        let mut dummy = ListNode::new(0);
        let mut curr = &mut dummy;

        let mut new_lists = vec![];
        for list in lists {
            if let Some(_) = list {
                new_lists.push(list);
            }
        }

        let len = new_lists.len();
        let mut none_idxs: Vec<bool> = vec![false; len];
        let mut set_none = 0;
        if new_lists.is_empty() {
            return None;
        }

        while set_none < len - 1  { // when set_none == len - 2, reserve a list
            let mut i_min = 0;
            let mut min = 10000;

            for i in 0..len {
                if none_idxs[i] {
                    continue;
                }
                if let Some(p) = new_lists[i].as_mut() {
                    if p.val < min {
                        i_min = i;
                        min = p.val;
                    }
                } else {
                    none_idxs[i] = true;
                    set_none += 1;
                }
            }

            curr.next = new_lists[i_min].take();
            curr = curr.next.as_mut().unwrap();
            new_lists[i_min] = curr.next.take();
        }


        if let Some(i) = none_idxs.iter().position(|&x| x == false) {
            curr.next = new_lists[i].take();
        }

        dummy.next
    }

    // beat 80% at speed, 63% at memory
    pub fn merge_k_lists1(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap: BinaryHeap<Box<ListNode>> = BinaryHeap::with_capacity(lists.len());
        for list in lists {
            if let Some(node) = list {
                heap.push(node);
            }
        }
        let mut dummy = ListNode::new(0);
        let mut curr = &mut dummy;
        while let Some(node) = heap.pop() {
            curr.next = Some(node);
            curr = curr.next.as_mut().unwrap();
            let new_node = curr.next.take();

            if new_node.is_some() {
                heap.push(new_node.unwrap());
            }
        }
        dummy.next
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::{helper, print_list, Solution};

    #[test]
    fn test1() {
        let v1 = vec![1, 4, 5];
        let v2 = vec![1, 3, 4];
        let v3 = vec![2, 6];
        let v4 = vec![3, 45, 100];
        let v5 = vec![7, 90, 91, 92, 93, 94, 95, 96, 97];

        let vv = vec![helper(v1), helper(v2), helper(v3), helper(v4), helper(v5)];

        let result = Solution::merge_k_lists(vv);
        print_list(&result);
    }

    #[test]
    fn test2() {
        let vv = vec![];
        let result = Solution::merge_k_lists(vv);
        print_list(&result);
    }

    #[test]
    fn test3() {
        let v1 = vec![];
        let vv = vec![helper(v1)];
        let result = Solution::merge_k_lists(vv);
        print_list(&result);
    }

    #[test]
    fn test4() {
        let v1 = vec![];
        let v2 = vec![];
        let vv = vec![helper(v1), helper(v2)];
        let result = Solution::merge_k_lists(vv);
        print_list(&result);
    }

    #[test]
    fn test5() {
        let v1 = vec![];
        let v2 = vec![1, 2, 3, 4];
        let v3 = vec![100, 101];
        let v4 = vec![];
        let vv = vec![helper(v1), helper(v2), helper(v3), helper(v4)];
        let result = Solution::merge_k_lists(vv);
        print_list(&result);
    }

    #[test]
    fn test6() {
        let v1 = vec![1, 4, 5];
        let v2 = vec![1, 3, 4];
        let v3 = vec![2, 6];
        let v4 = vec![3, 45, 100];
        let v5 = vec![7, 90, 91, 92, 93, 94, 95, 96, 97];

        let vv = vec![helper(v1), helper(v2), helper(v3), helper(v4), helper(v5)];

        let result = Solution::merge_k_lists1(vv);
        print_list(&result);
    }

    #[test]
    fn test7() {
        let v1 = vec![];
        let v2 = vec![1, 2, 3, 4];
        let v3 = vec![100, 101];
        let v4 = vec![];
        let vv = vec![helper(v1), helper(v2), helper(v3), helper(v4)];
        let result = Solution::merge_k_lists1(vv);
        print_list(&result);
    }

    #[test]
    fn test8() {
        let vv = vec![];
        let result = Solution::merge_k_lists1(vv);
        print_list(&result);
    }

    #[test]
    fn test9() {
        let v1 = vec![];
        let vv = vec![helper(v1)];
        let result = Solution::merge_k_lists1(vv);
        print_list(&result);
    }

    #[test]
    fn test10() {
        let v1 = vec![];
        let v2 = vec![];
        let vv = vec![helper(v1), helper(v2)];
        let result = Solution::merge_k_lists1(vv);
        print_list(&result);
    }


    #[test]
    fn test() {
        // let v = vec![1, 2, 3, 4, 5, 6, 7];
        // println!("{}", v.iter().find(4));
    }
}

