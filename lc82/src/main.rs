
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
        let mut current = &mut dummy;

        while let Some(mut node) = current.next.take() {
            let mut count = 1;
            let mut next = node.next.take();

            while let Some(mut next_node) = next.take() {
                if next_node.val == node.val {
                    count += 1;
                    next = next_node.next.take();
                } else {
                    break;
                }
            }

            if count == 1 {
                current.next = Some(node);
                current = current.next.as_mut().unwrap();
            } else {
                current.next = next;
            }
        }

        dummy.next
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

#[cfg(test)]
mod tests {
    use crate::{
        Solution,
        helper,
        print_list,
    };

    #[test]
    fn test1() {
        let v = vec![1, 2, 3, 3, 4, 4, 5];
        let head = helper(v);
        let result = Solution::delete_duplicates(head);
        print_list(&result);
    }

    #[test]
    fn test2() {
        let v = vec![1, 1, 1, 2, 3];
        let head = helper(v);
        let result = Solution::delete_duplicates(head);
        print_list(&result);
    }
}

fn main() {
    println!("Hello, world!");
}
