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
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if list1.is_none() {
            return list2;
        }
        if list2.is_none() {
            return list1;
        }

        let mut dummy = ListNode::new(0);
        let mut curr = &mut dummy;

        loop {
            match (list1.as_mut(), list2.as_mut()) {
                (Some(p1), Some(p2)) => {
                    if p1.val < p2.val {
                        curr.next = list1.take();
                        curr = curr.next.as_mut().unwrap();
                        list1 = curr.next.take();
                    } else {
                        curr.next = list2.take();
                        curr = curr.next.as_mut().unwrap();
                        list2 = curr.next.take();
                    }
                }
                _ => {
                    break;
                }
            }
        }

        curr.next = list1.or(list2); // or
        dummy.next
    }

    pub fn merge_two_lists1(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if list1.is_none() {
            return list2
        } else if list2.is_none() {
            return list1;
        }

        let mut dummy = Box::new(ListNode::new(0));
        let mut curr = &mut dummy;

        while let (Some(p1), Some(p2)) = (list1.as_mut(), list2.as_mut()) {
            if p1.val < p2.val {
                curr.next = list1.take();
                curr = curr.next.as_mut().unwrap();
                list1 = curr.next.take();
            } else {
                curr.next = list2.take();
                curr = curr.next.as_mut().unwrap();
                list2 = curr.next.take();
            }
        }

        curr.next = list1.or(list2);

        dummy.next
    }
    pub fn merge_two_lists2(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut prehead = ListNode::new(-1);
        let mut cur_node = &mut prehead;

        while let (Some(node1), Some(node2)) = (&list1, &list2) {
            if node1.val < node2.val {
                cur_node.next = list1.take();
                cur_node = cur_node.next.as_mut().unwrap();
                list1 = cur_node.next.take();
            } else {
                cur_node.next = list2.take();
                cur_node = cur_node.next.as_mut().unwrap();
                list2 = cur_node.next.take();
            }
        }
        cur_node.next = list1.or(list2);
        prehead.next
    }
}

fn main() {
    println!("Hello, world!");
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
    use crate::{helper, print_list, Solution};

    #[test]
    fn test1() {
        let v1 = vec![1, 2, 4];
        let v2 = vec![1, 3, 4];
        let l1 = helper(v1);
        let l2 = helper(v2);
        let result = Solution::merge_two_lists(l1, l2);
        print_list(&result);
    }

    #[test]
    fn test2() {
        let v1 = vec![];
        let v2 = vec![];
        let l1 = helper(v1);
        let l2 = helper(v2);
        let result = Solution::merge_two_lists(l1, l2);
        print_list(&result);
    }

    #[test]
    fn test3() {
        let v1 = vec![];
        let v2 = vec![0];
        let l1 = helper(v1);
        let l2 = helper(v2);
        let result = Solution::merge_two_lists(l1, l2);
        print_list(&result);
    }

    #[test]
    fn test4() {
        let v1 = vec![1, 2, 4];
        let v2 = vec![1, 3, 4];
        let l1 = helper(v1);
        let l2 = helper(v2);
        let result = Solution::merge_two_lists1(l1, l2);
        print_list(&result);
    }

    #[test]
    fn test5() {
        let v1 = vec![];
        let v2 = vec![];
        let l1 = helper(v1);
        let l2 = helper(v2);
        let result = Solution::merge_two_lists1(l1, l2);
        print_list(&result);
    }

    #[test]
    fn test6() {
        let v1 = vec![];
        let v2 = vec![0];
        let l1 = helper(v1);
        let l2 = helper(v2);
        let result = Solution::merge_two_lists1(l1, l2);
        print_list(&result);
    }
}
