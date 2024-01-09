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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32
    {
        if let Some(ref node) = root
        {
            let mut ref_node    = node.borrow_mut();
            let right           = Solution::max_depth(ref_node.right.take()) + 1;
            let left            = Solution::max_depth(ref_node.left.take()) + 1;
            right.max(left)
        }
        else
        {
            0
        }
    }
}