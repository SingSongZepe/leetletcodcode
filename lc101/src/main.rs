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

use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
	pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
		fn is_mirror(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
			if left.is_none() && right.is_none() {
				return true;
			}
			if left.is_none() || right.is_none() {
				return false;
			}
			if left.as_ref().unwrap().borrow().val == right.as_ref().unwrap().borrow().val {
				return is_mirror(left.as_ref().unwrap().borrow().left.clone(), right.as_ref().unwrap().borrow().right.clone())
					&& is_mirror(left.as_ref().unwrap().borrow().right.clone(), right.as_ref().unwrap().borrow().left.clone());
			}
			false
		}
		is_mirror(root.as_ref().unwrap().borrow().left.clone(), root.as_ref().unwrap().borrow().right.clone())
	}
}

fn helper(v: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
	fn build_tree(v: &Vec<i32>, i: usize) -> Option<Rc<RefCell<TreeNode>>> {
		if i >= v.len() || v[i] == 101 {
			return None;
		}
		let mut node = TreeNode::new(v[i]);
		node.left = build_tree(v, 2*i+1);
		node.right = build_tree(v, 2*i+2);
		Some(Rc::new(RefCell::new(node)))
	}
	build_tree(&v, 0)
}



fn main() {
    println!("Hello, world!");
}
