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

// recursive solution
impl Solution {
	pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
		let mut v = Vec::new();
		Solution::recu(root, &mut v);
		v
	}
	fn recu(node: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
		if let Some(node) = node {
			Solution::recu(node.borrow().left.clone(), v);
			v.push(node.borrow().val);
			Solution::recu(node.borrow().right.clone(), v);
		}
	}
}

struct Solution1;
// iterative solution
impl Solution1 {
	pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
		let mut result = Vec::new();
		let mut stack = Vec::new();
		let mut current = root;

		while current.is_some() || !stack.is_empty() {
			while let Some(node) = current {
				stack.push(Some(node.clone()));
				current = node.borrow().left.clone();
			}

			if let Some(node) = stack.pop() {
				let node_ref = node.unwrap();
				result.push(node_ref.borrow().val);
				current = node_ref.borrow().right.clone();
			}
		}

		result
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

fn main() {
    println!("Hello, world!");
}
