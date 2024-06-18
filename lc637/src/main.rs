mod test;

struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
	pub val: i32,
	pub left: Option<Rc<RefCell<TreeNode>>>,
	pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
	#[inline]
	pub fn new(val: i32) -> Self {
		TreeNode {
			val,
			left: None,
			right: None
		}
	}
	fn from_vec(vec: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
		fn build_tree(vec: &Vec<i32>, idx: usize) -> Option<Rc<RefCell<TreeNode>>> {
			if idx >= vec.len() {
				return None;
			}
			if vec[idx] == -1 {
				return None;
			}
			let mut node = Rc::new(RefCell::new(TreeNode::new(vec[idx])));
			node.borrow_mut().left = build_tree(vec, 2 * idx + 1);
			node.borrow_mut().right = build_tree(vec, 2 * idx + 2);
			Some(node)
		}
		build_tree(vec, 0)
	}
}

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::iter::once;

impl Solution {
	pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
		let mut q: VecDeque<_> = once(root).flatten().collect();
		let mut rez = vec![];
		while !q.is_empty() {
			let (mut sum, n) = (0.0, q.len());
			for _ in 0..n {
				let node_rc = q.pop_front().unwrap();
				let mut node_ref = node_rc.borrow_mut();
				sum += node_ref.val as f64;
				q.extend(once(node_ref.left.take()).chain(once(node_ref.right.take())).flatten());
			}
			rez.push(sum / (n as f64));
		}
		rez
	}
}

fn main() {
    println!("Hello, world!");
}
