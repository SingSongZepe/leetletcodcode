mod test;

struct Solution;

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

impl Solution {
	pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
		let mut stack = Vec::new();
		let mut curr = head.as_ref();
		while let Some(n) = curr {
			stack.push(n.val);
			curr = n.next.as_ref();
		}
		curr = head.as_ref();
		while let Some(n) = curr {
			if n.val!= stack.pop().unwrap() {
				return false;
			}
			curr = n.next.as_ref();
		}
		true
	}
}

fn helper(v: Vec<i32>) -> Option<Box<ListNode>> {
	let mut dummy = ListNode::new(0);
	let mut curr = &mut dummy;
	for i in v {
		curr.next = Some(Box::new(ListNode::new(i)));
		curr = curr.next.as_mut().unwrap();
	}
	Some(dummy.next.unwrap())
}

fn print_list(mut node: &Option<Box<ListNode>>) {
	while let Some(n) = node {
		print!("{} -> ", n.val);
		node = &n.next;
	}
	println!("");
}

fn main() {
    println!("Hello, world!");
}
