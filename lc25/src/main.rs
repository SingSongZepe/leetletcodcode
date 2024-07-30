// Definition for singly-linked list.
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

struct Solution;

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {


        head
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

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::{helper, ListNode, print_list, Solution};

    #[test]
    fn test1() {
        let v1 = vec![1, 2, 3, 4, 5];
        let k = 2;
        let l1 = helper(v1);
        print_list(&l1);

        let result = Solution::reverse_k_group(l1, k);
        print_list(&result);
    }

    #[test]
    fn test_clone1() {
        let mut i = 1;
        let mut b  = 1;
        b.clone_from(&i);
        b = 2;

        println!("{}", i);
        println!("{}", b);

    }

    #[test]
    fn test_clone2() {
        let mut i = helper(vec![1, 9999]);
        let mut b  = helper(vec![9]);
        b.clone_from(&i);
        b.unwrap().next.unwrap().val = 3;

        print_list(&i);
    }

}


