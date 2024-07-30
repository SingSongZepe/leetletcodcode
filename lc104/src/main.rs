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
}

use std::rc::Rc;
use std::cell::RefCell;

// DFS solution
impl Solution {
	pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		fn dfs(node: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
			if node.is_none() {
				return depth - 1;
			}
			dfs(node.as_ref().unwrap().borrow().left.clone(), depth+1)
				.max(dfs(node.as_ref().unwrap().borrow().right.clone(), depth+1))
		}
		dfs(root, 1)
	}
}

struct Solution1;

// BFS solution
use std::collections::VecDeque;

impl Solution1 {
	pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		if root.is_none() {
			return 0;
		}
		let mut queue = VecDeque::new();
		queue.push_back(root.clone());
		let mut depth = 0;
		while !queue.is_empty() {
			let size = queue.len();
			for _ in 0..size {
				if let Some(node) = queue.pop_front().unwrap() {
					if node.as_ref().borrow().left.is_some() {
						queue.push_back(node.borrow().left.clone());
					}
					if node.borrow().right.is_some() {
						queue.push_back(node.borrow().right.clone());
					}
				}
			}
			depth += 1;
		}
		depth
	}
}

fn helper(v: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
	fn build_tree(v: &Vec<i32>, i: usize) -> Option<Rc<RefCell<TreeNode>>> {
		if i >= v.len() || v[i] == 101 {
			return None;
		}
		let node = Rc::new(RefCell::new(TreeNode::new(v[i])));
		node.borrow_mut().left = build_tree(v, 2*i+1);
		node.borrow_mut().right = build_tree(v, 2*i+2);
		Some(node)
	}
	build_tree(&v, 0)
}

fn main() {
    println!("Hello, world!");
}
