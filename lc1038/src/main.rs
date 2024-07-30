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

type Node = Option<Rc<RefCell<TreeNode>>>;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
	pub fn bst_to_gst(root: Node) -> Node {
		fn dfs(node: &Node, sum: &mut i32) {
			if let Some(node) = node {
				let mut node = node.borrow_mut();
				dfs(&node.right, sum);
				*sum += node.val;
				node.val = *sum;
				dfs(&node.left, sum);
			}
		}
		let mut sum: i32 = 0;
		dfs(&root, &mut sum);
		root
	}
}


struct Solution1;
impl Solution1 {
	pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
		fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
			if let Some(node) = node {
				let mut node = node.borrow_mut();
				dfs(&node.right, sum);
				*sum += node.val;
				node.val = *sum;
				dfs(&node.left, sum);
			}
		}
		let mut sum: i32 = 0;
		dfs(&root, &mut sum);
		root
	}
}


fn helper(v: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
	fn build(v: &Vec<i32>, i: usize) -> Option<Rc<RefCell<TreeNode>>> {
		if i >= v.len() || v[i] == -1 {
			None
		} else {
			let mut node = Rc::new(RefCell::new(TreeNode::new(v[i])));
			node.borrow_mut().left = build(v, 2 * i + 1);
			node.borrow_mut().right = build(v, 2 * i + 2);
			Some(node)
		}
	}
	build(&v, 0)
}


fn main() {
    println!("Hello, world!");
}
