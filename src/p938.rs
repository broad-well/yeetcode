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

struct Solution;
// Copy from below
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        if let Some(root_val) = root {
            let node = root_val.borrow();
            let root_num = node.val;
            let mut answer = 0;
            // consider left subtree (all < root_num)
            if low < root_num {
                // left subtree (all < root) might have numbers in [low, high]
                answer += Solution::range_sum_bst(node.left.clone(), low, high);
            }
            // consider right subtree (all > root_num)
            if high > root_num {
                // right subtree (all > root_num) might have numbers in [low, high]
                answer += Solution::range_sum_bst(node.right.clone(), low, high);
            }
            if root_num >= low && root_num <= high {
                answer += root_num;
            }
            answer
        } else { 0 }
    }
}