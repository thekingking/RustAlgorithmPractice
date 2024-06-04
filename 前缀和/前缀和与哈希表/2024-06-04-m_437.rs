use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

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

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, cnt: &mut HashMap<i64, i32>, ans: &mut i32, pre: i64, target_sum: i64) {
            if let Some(node) = node {
                let node = node.borrow();
                let cur = pre + node.val as i64;
                *ans += cnt.get(&(cur - target_sum)).unwrap_or(&0);
                cnt.entry(cur).and_modify(|x| *x += 1).or_insert(1);
                dfs(&node.left, cnt, ans, cur, target_sum);
                dfs(&node.right, cnt, ans, cur, target_sum);
                cnt.entry(cur).and_modify(|x| *x -= 1);
            }
        }
        let mut ans = 0;
        let mut cnt = HashMap::new();
        cnt.insert(0, 1);
        dfs(&root, &mut cnt, &mut ans, 0, target_sum as i64);
        ans
    }
}