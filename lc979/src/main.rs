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
	pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		let mut step = 0;
		Self::make_balance(root, &mut step);
		step
	}
	fn make_balance(root: Option<Rc<RefCell<TreeNode>>>, step: &mut i32) -> i32 {
		if let Some(node) = root {
			let need = Self::make_balance(node.borrow().left.clone(), step) +
				Self::make_balance(node.borrow().right.clone(), step);
			*step += (need - node.borrow().val + 1).abs();
			return need - node.borrow().val + 1;
		} else {
			0
		}
	}
}

struct Solution1;

impl Solution1 {
	pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		let mut step = 0;
		Self::make_balance(root, &mut step);
		step
	}
	fn make_balance(root: Option<Rc<RefCell<TreeNode>>>, step: &mut i32) -> i32 {
		let n = root.unwrap();
		let mut need = 0;
		if let Some(left) = n.borrow().left.clone() {
			need += Self::make_balance(Some(left), step);
		}
		if let Some(right) = n.borrow().right.clone() {
			need += Self::make_balance(Some(right), step);
		}
		let need_for = need - n.borrow().val + 1;
		*step += need_for.abs();
		need_for
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



