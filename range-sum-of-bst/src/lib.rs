//! https://leetcode.com/problems/shuffle-string/

pub struct Solution {}

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
    fn range_sum_bst_impl(root: &Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let root: std::cell::Ref<TreeNode> = match root {
            None => return 0,
            Some(r) => (&r).borrow(),
        };
        let root_val = (*root).val;
        let mut result = root_val;
        if root_val < low {
            Solution::range_sum_bst_impl(&root.right, low, high)
        } else if root_val > high {
            Solution::range_sum_bst_impl(&root.left, low, high)
        } else {
            result += Solution::range_sum_bst_impl(&root.left, low, high);
            result += Solution::range_sum_bst_impl(&root.right, low, high);
            result
        }
    }
    
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        assert!(low <= high);
        return Solution::range_sum_bst_impl(&root, low, high);
    }
}
