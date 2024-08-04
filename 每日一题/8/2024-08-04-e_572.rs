// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
     pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if Self::is_same(root.clone(), sub_root.clone()) {
            return true;
        }
        match root {
            Some(root) => {
                Self::is_subtree(root.borrow().left.clone(), sub_root.clone()) ||
                Self::is_subtree(root.borrow().right.clone(), sub_root.clone())
            },
            None => false,
        }
    }

    fn is_same(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (root1, root2) {
            (None, None) => true,
            (Some(r1), Some(r2)) => {
                r1.borrow().val == r2.borrow().val &&
                Self::is_same(r1.borrow().left.clone(), r2.borrow().left.clone()) && 
                Self::is_same(r1.borrow().right.clone(), r2.borrow().right.clone())
            },
            _ => false,
        }
    }
}