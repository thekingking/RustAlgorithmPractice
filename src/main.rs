use std::{arch::x86_64, borrow::Borrow, cmp::{self, max}, collections::{BTreeMap, HashMap, HashSet}, i32, vec};

fn main() {
    println!("hello, world");
}

struct Solution;

impl Solution {
    
}

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
impl Solution {
    pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(&root, &sub_root)
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, sub_root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => false,
            Some(node) => {
                Self::is_same(root, sub_root) || 
                Self::dfs(&node.borrow().left, sub_root) ||
                Self::dfs(&node.borrow().right, sub_root)
            }
        }
    }

    fn is_same(root1: &Option<Rc<RefCell<TreeNode>>>, root2: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (root1, root2) {
            (None, None) => true,
            (Some(r1), Some(r2)) => {
                r1.borrow().val == r2.borrow().val &&
                Self::is_same(&r1.borrow().left, &r2.borrow().left) && 
                Self::is_same(&r1.borrow().right, &r2.borrow().right)
            },
            _ => false,
        }
    }
}