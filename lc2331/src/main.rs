
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
	pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
		if let Some(n) = root {
			if n.borrow().left.is_some() { // non-Leaf OR 2 AND 3
				let left = Self::evaluate_tree(n.borrow().left.clone());
				let right = Self::evaluate_tree(n.borrow().right.clone());
				match n.borrow().val {
					2 => left || right,
					3 => left && right,
					_ => panic!("unknown value of non-leaf node"),
				}
			} else { // Leaf True 1 False 0
				match n.borrow().val {
					1 => true,
					0 => false,
					_ => panic!("unknown value of leaf node"),
				}
			}
		} else {
			panic!("can't pass None to traverse")
		}
	}
}

struct Solution1;

impl Solution1 {
	pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
		if let Some(n) = root {
			match n.borrow().val {
				0 => false,
				1 => true,
				2 => Solution1::evaluate_tree(n.borrow().left.clone()) || Solution1::evaluate_tree(n.borrow().right.clone()),
				_ => Solution1::evaluate_tree(n.borrow().left.clone()) && Solution1::evaluate_tree(n.borrow().right.clone()),
			}
		} else {
			panic!("can't pass None to traverse")
		}
	}
}

// input: [2,1,3,-1,-1,0,1]
use std::collections::VecDeque;

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

fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) {
	fn traverse(node: &Option<Rc<RefCell<TreeNode>>>, level: usize) {
		if let Some(n) = node {
			println!("{}{}", "  ".repeat(level), n.borrow().val);
			traverse(&n.borrow().left, level + 1);
			traverse(&n.borrow().right, level + 1);
		}
	}

	traverse(&root, 0);
}

fn main() {
    println!("Hello, world!");
}

