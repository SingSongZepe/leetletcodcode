mod test;

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
	pub fn solve() {
		
	}
}

struct Reverse;

impl Reverse {
	pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		if head.as_ref().and_then(|node| node.next.as_ref()).is_none() {
			return head;
		}

		let mut new_head = Self::reverse_list(head.as_mut().and_then(|node| node.next.take()));
		head.as_mut().and_then(|node| {
			let mut next_node = node.next.take();
			if let Some(n) = next_node.as_mut() {
				n.next = Some(node.clone());
			}
			new_head.as_mut().map(|node| {
				node.as_mut().next = next_node;
			})
		});
		new_head
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
	if head.is_none() {
		println!("None");
		return;
	}
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
