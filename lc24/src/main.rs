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
    // good usage of std::mem::swap
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() { return None; }
        let mut head_pointer = &mut head;
        while let Some(node) = head_pointer {
            if node.next.is_none() { return head; }
            let mut temp = node.next.as_mut().unwrap().next.clone();
            std::mem::swap(&mut node.next, &mut temp);
            temp.as_mut().unwrap().next = Some(Box::new(*node.clone()));
            std::mem::swap(node, &mut temp.as_mut().unwrap());
            head_pointer = &mut node.next.as_mut().unwrap().next;
        }
        head
    }

    // for test
    pub fn swap_pairs1(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() { return None; }
        let mut head_pointer = &mut head;
        while let Some(node) = head_pointer {
            if node.next.is_none() { return head; }
            let mut temp = node.next.as_mut().unwrap().next.clone();

            println!("swap 1");
            print_list1(Some(&node));
            print_list1(temp.as_ref());
            println!("");

            std::mem::swap(&mut node.next, &mut temp);

            println!("swap 2");
            print_list1(Some(&node));
            print_list1(temp.as_ref());
            println!("");

            temp.as_mut().unwrap().next = Some(Box::new(*node.clone()));

            println!("swap 3");
            print_list1(Some(&node));
            print_list1(temp.as_ref());
            println!("");

            std::mem::swap(node, &mut temp.as_mut().unwrap());

            println!("swap 4");
            print_list1(Some(&node));
            print_list1(temp.as_ref());
            println!("");

            head_pointer = &mut node.next.as_mut().unwrap().next;
        }
        head
    }

    // very well algorithm
    pub fn swap_pairs2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(mut h) => {
                match h.next {
                    Some(mut n) => {
                        h.next = Solution::swap_pairs(n.next);
                        n.next = Some(h);
                        Some(n)
                    }
                    None => Some(h),
                }
            }
            None => head,
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

fn print_list1(head: Option<&Box<ListNode>>) {
    let mut current = head;
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
    use crate::{helper, ListNode, print_list, print_list1, Solution};

    #[test]
    fn test1() {
        let v = vec![1,2,3,4];
        let result = Solution::swap_pairs1(helper(v));
        print_list(&result);
    }

    #[test]
    fn test2() {
        let v = vec![];
        let result = Solution::swap_pairs(helper(v));
        print_list(&result);
    }

    #[test]
    fn test3() {
        // let v = vec![1,2,3,4];
        let result = Solution::swap_pairs(None);
        print_list(&result);
    }

    #[test]
    fn test4() {
        let v = vec![1,2,3,4,5];
        let result = Solution::swap_pairs1(helper(v));
        print_list(&result);
    }

    #[test]
    fn test5() {
        let v = vec![1];
        let result = Solution::swap_pairs1(helper(v));
        print_list(&result);
    }

    #[test]
    fn test6() {
        let v = vec![1, 2];
        let result = Solution::swap_pairs1(helper(v));
        print_list(&result);
    }

    #[test]
    fn test7() {
        let v = vec![1, 2, 3];
        let result = Solution::swap_pairs2(helper(v));
        print_list(&result);
    }

    #[test]
    fn test() {
        let a = Some(10);
        let b = a.and_then(|x| Some(x+1));
        println!("{}", a.unwrap());
        println!("{}", b.unwrap());

        let mut c = ListNode::new(100);
        let mut d = ListNode::new(999);
        std::mem::swap(&mut c, &mut d);

        println!("c: {} | d: {}", c.val, d.val);


    }

    #[test]
    fn test_swap1() {
        let mut l1 = helper(vec![1, 2, 3, 4]);
        if let Some(node) = l1.as_mut() {
            let mut temp = node.next.as_mut().unwrap().clone();

            // print_list(&Some(node.to_owned()));
            // print_list(&Some(temp));

            std::mem::swap(node, &mut temp);
            print_list(&Some(node.to_owned()));
            print_list(&Some(temp));
        }
    }

    #[test]
    fn test_swap2() {
        let mut l1 = helper(vec![1, 3, 4]);
        if let Some(node) = l1.as_mut() {
            let mut temp = node.next.as_mut().unwrap().next.as_mut().unwrap().clone();
            print_list1(Some(node));
            print_list1(Some(&temp));

            std::mem::swap(node, &mut temp);

            print_list1(Some(node));
            print_list1(Some(&temp));
        }
    }
}
