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
	pub fn convert_bst(root: Node) -> Node {
		fn dfs(root: &Node, sum: &mut i32) {
			if let Some(node) = root {
				let mut node = node.borrow_mut();
				dfs(&node.right, sum);
				*sum += node.val;
				node.val = *sum;
				dfs(&node.left, sum);
			}
		}
		dfs(&root, &mut 0);
		root
	}
}

// helper function
// replace null with -1
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
