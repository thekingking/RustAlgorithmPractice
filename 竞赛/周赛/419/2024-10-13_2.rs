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
    pub fn kth_largest_perfect_subtree(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) -> (bool, i32) {
            if let Some(node) = node {
                let node = node.borrow();
                let (left, l) = dfs(node.left.clone(), arr);
                let (right, r) = dfs(node.right.clone(), arr);
                if !left || !right || l != r {
                    return (false, 0);
                }
                arr.push(l + r + 1);
                (true, l + r + 1)
            } else {
                (true, 0)
            }
        }
        let mut arr = vec![];
        dfs(root, &mut arr);
        arr.sort_unstable();
        if arr.len() < k as usize {
            -1
        } else {
            arr[arr.len() - k as usize]
        }
    }
}