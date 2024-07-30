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
	pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
		let mut v = Vec::new();
		Self::in_order_traversal(root, &mut v)
	}

	fn in_order_traversal(node: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) -> bool {
		if let Some(n) = node {
			let n_ref = n.as_ref().borrow();
			if !Self::in_order_traversal(n_ref.left.clone(), v) {
				return false;
			}
			if !v.is_empty() && n_ref.val <= v[v.len() - 1] {
				return false;
			}
			v.push(n_ref.val);
			Self::in_order_traversal(n_ref.right.clone(), v)
		} else {
			true
		}
	}
}

struct Solution1;

type OptNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution1 {
	pub fn is_valid_bst(root: OptNode) -> bool {
		Self::is_valid(&root, i32::MIN as i64 - 1, i32::MAX as i64 + 1)
	}

	fn is_valid(node: &OptNode, gt: i64, lt: i64) -> bool {
		match node.as_ref() {
			None => true,
			Some(n) => {
				let b = n.borrow();
				(b.val as i64) > gt && (b.val as i64) < lt
					&& Self::is_valid(&b.left, gt, b.val as i64)
					&& Self::is_valid(&b.right, b.val as i64, lt)
			}
		}
	}
}


fn helper(v: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
	fn build(v: &Vec<i32>, idx: usize) -> Option<Rc<RefCell<TreeNode>>> {
		if idx >= v.len() || v[idx] == -1 {
			return None;
		}

		let mut node = TreeNode::new(v[idx]);
		node.left = build(v, 2 * idx + 1);
		node.right = build(v, 2 * idx + 2);

		Some(Rc::new(RefCell::new(node)))
	}

	build(&v, 0)
}

fn helper1(v: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
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


