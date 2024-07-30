mod test;

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
use std::collections::VecDeque;
impl Solution {
	// output the sum of the max level,
	// but we need to find the max level and return
	pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		let mut vqueue = VecDeque::new();
		vqueue.push_back((root, 1));

		let mut m = i32::MIN;
		let mut curr = 0;
		let mut curr_level = 1;

		while !vqueue.is_empty() {
			if let Some((n, level)) = vqueue.pop_front() {
				let nx = n.unwrap();
				if nx.borrow().left.is_some() {
					vqueue.push_back((nx.borrow().left.clone(), level+1));
				}
				if nx.borrow().right.is_some() {
					vqueue.push_back((nx.borrow().right.clone(), level+1));
				}
				if curr_level != level {
					m = m.max(curr);
					curr = 0;
					curr_level = level;
				}
				curr += nx.borrow().val;
			}
		}
		m.max(curr)
	}
}

struct Solution1;

impl Solution1 {
	// output the sum of the max level,
	// but we need to find the max level and return
	pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		let mut vqueue = VecDeque::new();
		vqueue.push_back((root, 1));

		let mut m = i32::MIN;
		let mut curr = 0;
		let mut m_level = 1;
		let mut curr_level = 1;

		while !vqueue.is_empty() {
			if let Some((n, level)) = vqueue.pop_front() {
				let nx = n.unwrap();
				if nx.borrow().left.is_some() {
					vqueue.push_back((nx.borrow().left.clone(), level+1));
				}
				if nx.borrow().right.is_some() {
					vqueue.push_back((nx.borrow().right.clone(), level+1));
				}
				if curr_level != level {
					if curr > m {
						m = curr;
						m_level = curr_level;
					}
					curr = 0;
					curr_level = level;
				}
				curr += nx.borrow().val;
			}
		}
		if curr > m {
			curr_level
		} else {
			m_level
		}
	}
}


struct Solution2;

impl Solution2 {
	pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		let mut vqueue = VecDeque::new();
		vqueue.push_back(root.unwrap());

		let mut m = i32::MIN;
		let mut curr = 0;
		let mut curr_level = 1;
		let mut level = 1;

		while !vqueue.is_empty() {
			let mut next_level = Vec::new();
			for node in vqueue {
				let nx = node.borrow();
				curr += nx.val;
				if nx.left.is_some() {
					next_level.push(Rc::clone(&nx.left.as_ref().unwrap()));
				}
				if nx.right.is_some() {
					next_level.push(Rc::clone(&nx.right.as_ref().unwrap()));
				}
			}
			if curr > m {
				m = curr;
				level = curr_level;
			}
			curr = 0;
			curr_level += 1;
			vqueue = VecDeque::from(next_level);
		}
		level
	}
}


fn helper(v: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
	fn build_tree(v: &Vec<Option<i32>>, index: usize) -> Option<Rc<RefCell<TreeNode>>> {
		if index >= v.len() || v[index].is_none() {
			return None;
		}

		let node = Rc::new(RefCell::new(TreeNode::new(v[index].unwrap())));
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

fn make_vec_opt(v: Vec<i32>) -> Vec<Option<i32>> {
	let mut vs = vec![];
	for i in v {
		if i == i32::MIN {
			vs.push(None);
		} else {
			vs.push(Some(i));
		}
	}
	vs
}

fn main() {
    println!("Hello, world!");
}
