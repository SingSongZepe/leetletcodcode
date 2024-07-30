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
	pub fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let r = root.unwrap();
        Self::recu(Some(r.clone()), target);
        if r.borrow().left.is_none() && r.borrow().right.is_none() && r.borrow().val == target {
            return None;
        }
        Some(r)
    }
    fn recu(node: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let n = node.unwrap();
        let mut nx = n.borrow_mut();
        if nx.left.is_some() {
            nx.left = Self::recu(nx.left.clone(), target);
        }
        if nx.right.is_some() {
            nx.right = Self::recu(nx.right.clone(), target);
        }
        if nx.left.is_none() && nx.right.is_none() && nx.val == target {
            return None;
        }
        return Some(n.clone());
    }
}

struct Solution1;

impl Solution1 {
    pub fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let n = root.unwrap();
        let mut nx = n.borrow_mut();
        nx.left = Self::remove_leaf_nodes(nx.left.clone(), target);
        nx.right = Self::remove_leaf_nodes(nx.right.clone(), target);
        if nx.left.is_none() && nx.right.is_none() && nx.val == target {
            return None;
        }
        Some(n.clone())
    }
}

struct Solution2;


impl Solution2 {
    pub fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let mut nx = node.borrow_mut();
            nx.left = Self::remove_leaf_nodes(nx.left.clone(), target);
            nx.right = Self::remove_leaf_nodes(nx.right.clone(), target);
            if nx.left.is_none() && nx.right.is_none() && nx.val == target {
                return None;
            }
            Some(node.clone())
        } else {
            None
        }
    }
}

struct Solution3;

impl Solution3 {
    pub fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = vec![root.clone()];
        let mut result = root.clone();

        while let Some(node) = stack.pop() {
            if let Some(n) = node {
                let mut nx = n.borrow_mut();
                if let Some(left) = nx.left.clone() {
                    if let Some(left_node) = left.borrow().clone() { // !
                        if left_node.borrow().val == target && left_node.borrow().left.is_none() && left_node.borrow().right.is_none() {
                            nx.left = None;
                        } else {
                            stack.push(nx.left.clone());
                        }
                    }
                }

                if let Some(right) = nx.right.clone() {
                    if let Some(right_node) = right.borrow().clone() {
                        if right_node.borrow().val == target && right_node.borrow().left.is_none() && right_node.borrow().right.is_none() {
                            nx.right = None;
                        } else {
                            stack.push(nx.right.clone());
                        }
                    }
                }
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
