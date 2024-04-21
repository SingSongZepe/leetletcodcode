use std::thread::current;

#[derive(PartialEq, Eq, Clone, Debug, Default)]
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
    // pub fn remove_nth_from_end_(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    //     if let Some(node) = head.as_ref() {
    //         if node.next.is_none() {
    //             return None;
    //         }
    //     }
    //
    //     let mut refs: Vec<&mut Box<ListNode>> = vec![];
    //     let mut current = &mut head;
    //
    //     while let Some(_) = current {
    //
    //     }
    //     if (refs.len() as i32) < n {
    //         panic!("len < n");
    //     }
    //
    //     refs.reverse();
    //
    //     refs[n as usize - 1].next = refs[n as usize].next.take();
    //
    //     head
    // }

    // pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    //     if let Some(node) = head.as_ref() {
    //         if node.next.is_none() {
    //             return None;
    //         }
    //     }
    //
    //     let mut len = 0;
    //     let mut current = &mut head;
    //     while let Some(_) = current {
    //         len += 1;
    //         current = &mut current.as_mut().unwrap().next;
    //     }
    //
    //     current = &mut head;
    //     let mut prev: Option<&mut ListNode> = None;
    //     while len > n {
    //         prev = current.as_mut().map(|node| &mut **node);
    //         current = &mut current.as_mut().as_mut().unwrap().next;
    //         len -= 1;
    //     }
    //
    //     if let Some(prev_node) = prev {
    //         if let Some(next_node) = current.as_ref().and_then(|node| node.next.as_ref()) {
    //             let next = std::mem::replace(&mut prev_node.next, None);
    //             prev_node.next = next_node.next.take();
    //         }
    //     }
    //
    //     head
    // }
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let cnt = std::iter::successors(head.as_ref(), |last| last.next.as_ref()).count();
        let mut dummy = Some(Box::new(ListNode{val: 0, next: head}));
        let mut prev = (0..cnt - n as usize).fold(dummy.as_mut(), |curr, _| curr.unwrap().next.as_mut() );
        prev.unwrap().next = prev.as_mut().unwrap().next.as_mut().unwrap().next.take();
        dummy.unwrap().next
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

pub fn print_list1(head: &Box<ListNode>) {
    print!("{} ", head.val);

    // 递归调用打印下一个节点的值
    if let Some(next) = &head.next {
        print_list1(next);
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::{helper, print_list, print_list1, Solution};

    #[test]
    fn test() {
        let v = vec![1,2,3,4,5];
        let mut r = helper(v);
        let n = 2;
        let len = 5;
        print_list(&r);

        let curr = (0..len-n).fold(r.as_mut(), |curr, _| curr.unwrap().next.as_mut());
        // print_list(&curr);

        println!();


        let ta = (0..100).take(len-n);
        ta.into_iter().map(move |c| print!(" {} ", c)).for_each(drop);


        let mut iter = std::iter::successors(Some(1), |x| Some(2 * x));
        println!("{:?}", iter.next());  // Some(0)
        println!("{:?}", iter.next());  // Some(1)
        println!("{:?}", iter.next());  // Some(2)

        // let len = iter.count();
        // println!("{}", len);

        let mut iter = std::iter::successors(r.as_ref(), |r| r.next.as_ref());
        iter.next();
        iter.next();
        iter.next();
        iter.next(); // 4

        // iter.next(); // 5
        println!("{}", iter.next().expect("error").val);
    }

    #[test]
    fn test1() {
        let v = vec![1,2,3,4,5];
        let r = helper(v);
        let n = 2;
        print_list(&r);
        let rr = Solution::remove_nth_from_end(r, n);
        print_list(&rr);
    }


}
