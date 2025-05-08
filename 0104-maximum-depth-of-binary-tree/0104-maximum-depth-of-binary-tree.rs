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
    pub fn max_depth(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node_rc) = root {
            let mut node = node_rc.borrow_mut();

            1 + Self::max_depth(node.left.take()).max(Self::max_depth(node.right.take()))
        } else {
            0
        }
    }
}