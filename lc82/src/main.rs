mod test;

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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        dummy.next = head;
        let mut curr = &mut dummy;


        dummy.next
    }
}

struct Solution1;

// use std::collections::HashSet;

impl Solution1 {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut v = vec![];
        let binding = head.unwrap();
        let mut current = binding.as_ref();
        while current.next.is_some() {
            v.push(current.val);
            current = current.next.as_ref().unwrap();
        }
        v.push(current.val);
        Self::make_list(v.iter().filter(|&x| v.iter().filter(|&y| y == x).count() == 1).cloned().collect())
    }
    pub fn make_list(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut current = head.as_mut().unwrap();
        for val in v {
            current.next = Some(Box::new(ListNode::new(val)));
            current = current.next.as_mut().unwrap();
        }
        head.unwrap().next
    }
}

struct Solution2;

impl Solution2 {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut v = vec![];
        let binding = head.unwrap();
        let mut curr = binding.as_ref();
        let mut equal = false;
        while curr.next.is_some() {
            let (val1, val2) = (curr.val, curr.next.as_ref().unwrap().val);
            if val1 == val2 {
                equal = true;
            } else {
                if !equal {
                    v.push(curr.val);
                    // equal = false;
                }
                equal = false;
            }
            curr = curr.next.as_ref().unwrap();
        }
        // last element
        if !equal {
            v.push(curr.val);
        }
        Self::make_list(v)
    }
    pub fn make_list(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut current = head.as_mut().unwrap();
        for val in v {
            current.next = Some(Box::new(ListNode::new(val)));
            current = current.next.as_mut().unwrap();
        }
        head.unwrap().next
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
