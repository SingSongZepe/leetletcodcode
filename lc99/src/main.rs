mod test;

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
}

struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
	pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {

	}
}

struct Solution1;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution1 {
	pub fn recover_tree(root: &mut Node) {
		let mut first = Node::None;
		let mut second = Node::None;
		let mut previous_node = Node::None;

		let mut current_node = root.as_ref().map(Rc::clone);
		let mut stack = vec![];
		while current_node.is_some() || !stack.is_empty() {
			if let Some(node) = current_node {
				// Keep going left until we fall off the bottom left edge of this subtree
				stack.push(Rc::clone(&node));
				current_node = node.borrow().left.as_ref().map(Rc::clone);
			} else if let Some(node) = &stack.pop() {
				// Take last value put onto stack and perform logic here
				let node_val = node.borrow().val;

				if let Some(pre_node) = &previous_node {
					let pre_val = pre_node.borrow().val;
					if pre_val > node_val {
						if first.is_none() {
							first = Some(Rc::clone(pre_node));
							second = Some(Rc::clone(node)); // Done here be the adjacent node might be the one to switch with
						} else {
							second = Some(Rc::clone(node));
							break; // Based on question there is exactly one switch to be made, no point checking the rest of the nodes
						}
					}
				}

				previous_node = Some(Rc::clone(node));
				current_node = node.borrow().right.as_ref().map(Rc::clone); // Setup for next iteration, will go to bottom left of this tree if exists else it will go up the stack
			}
		}

		if let (Some(first), Some(second)) = (first, second) {
			std::mem::swap(&mut first.borrow_mut().val, &mut second.borrow_mut().val);
		}
	}
}

fn helper(v: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
	fn build_tree(v: &Vec<i32>, index: usize) -> Option<Rc<RefCell<TreeNode>>> {
		if index >= v.len() || v[index] == -1 {
			return None;
		}
		let node = Rc::new(RefCell::new(TreeNode::new(v[index])));
		if let Some(ref mut left) = build_tree(v, 2 * index + 1) {
			node.borrow_mut().left = Some(left.clone());
		}
		if let Some(ref mut right) = build_tree(v, 2 * index + 2) {
			node.borrow_mut().right = Some(right.clone());
		}
		Some(node)
	}
	build_tree(&v, 0)
}

fn collect_tree(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
	let mut result = Vec::new();
	collect_helper(root, &mut result);
	result
}

fn collect_helper(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
	if let Some(node) = root {
		let node_ref = node.borrow();
		collect_helper(&node_ref.left, result);
		result.push(node_ref.val);
		collect_helper(&node_ref.right, result);
	}
}

fn main() {
    println!("Hello, world!");
}



