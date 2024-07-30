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
	pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
		// let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
		// let mut prev = &mut dummy;
		//
		// for _ in 0..left-1 {
		// 	if let Some(ref mut node) = prev {
		// 		prev = &mut node.next;
		// 	}
		// }
		//
		// let mut curr = prev.as_mut().unwrap().next.take();
		// let mut next = curr.as_mut().unwrap().next.take();
		//
		// for _ in left..right {
		// 	curr.as_mut().unwrap().next = next.as_mut().unwrap().next.take();
		// 	next.as_mut().unwrap().next = prev.as_mut().unwrap().next.take();
		// 	prev.as_mut().unwrap().next = next.take();
		// 	next = curr.as_mut().unwrap().next.take();
		// }
		//
		// let mut tail = prev.as_mut().unwrap().next.take();
		// while let Some(mut node) = tail {
		// 	tail = node.next.take();
		// 	prev = &mut tail;
		// }
		// prev.as_mut().unwrap().next = next;
		//
		// dummy.unwrap().next
		None
	}
}


struct Solution1;

impl Solution1 {
	fn reverse_between(mut head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
		let mut v = vec![];
		while let Some(node) = head.take() {
			v.push(node.val);
			head = node.next;
		}
		Self::swap_elements(&mut v, left as usize - 1, right as usize - 1);
		Self::make_list(v)
	}
	fn swap_elements(vec: &mut Vec<i32>, left: usize, right: usize) {
		(left..(left+right)/2).zip(((left+right)/2+1..=right).rev()).for_each(|(i, j)| {
			vec.swap(i, j);
		});
	}
	fn make_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
		let mut head = Some(Box::new(ListNode::new(0)));
		let mut current = head.as_mut().unwrap();

		for val in nums {
			current.next = Some(Box::new(ListNode::new(val)));
			current = current.next.as_mut().unwrap();
		}
		head.unwrap().next
	}
}


struct Solution2;


impl Solution2 {
	// beautiful solution
	pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
		let mut fake_head = Some(Box::new(ListNode { val: 0, next: head }));
		let head = fake_head.as_mut();
		if left >= right {
			return fake_head?.next;
		}

		let mut body = head;
		for _ in 1..left {
			body = body?.next.as_mut();
		}
		let raw_tail = Box::into_raw(body.as_mut()?.next.take()?);
		let mut fuse = unsafe { Some(Box::from_raw(raw_tail)) };
		let mut tail = fuse.as_mut()?.next.take();
		body.as_mut()?.next = fuse;
		for _ in left..right {
			let lhs = body.as_mut()?.next.take();
			let mut rhs = tail;
			let next_tail = rhs.as_mut()?.next.take();
			rhs.as_mut()?.next = lhs;
			body.as_mut()?.next = rhs;
			tail = next_tail;
		}
		unsafe { (*raw_tail).next = tail }
		fake_head?.next
	}
}


struct Reverse;

impl Reverse {
	pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		Self::reverse(None, head)
	}

	fn reverse(prev: Option<Box<ListNode>>, curr: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		if let Some(mut node) = curr {
			let next = node.next.take();
			node.next = prev;
			Self::reverse(Some(node), next)
		} else {
			prev
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
