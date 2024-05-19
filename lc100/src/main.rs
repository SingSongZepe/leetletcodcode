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
	pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
		if p.is_some() && q.is_some() {
			let np = p.unwrap();
			let nq = q.unwrap();
			return np.borrow().val == nq.borrow().val &&
				Solution::is_same_tree(np.borrow().left.clone(), nq.borrow().left.clone()) &&
				Solution::is_same_tree(np.borrow().right.clone(), nq.borrow().right.clone());
		} else if p.is_none() && q.is_none() {
			true
		} else {
			false
		}
	}
}

struct Solution1;

impl Solution1 {
	pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
		p.as_ref().map_or(q.is_none(), |np|
			q.as_ref().map_or(false, |nq|
				np.borrow().val == nq.borrow().val &&
				Solution::is_same_tree(np.borrow().left.clone(), nq.borrow().left.clone()) &&
				Solution::is_same_tree(np.borrow().right.clone(), nq.borrow().right.clone())
			)
		)
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
