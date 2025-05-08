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
        Solution::is(root.as_ref(), sub_root.as_ref())
    }

    fn is(root: Option<&Rc<RefCell<TreeNode>>>, sub_root: Option<&Rc<RefCell<TreeNode>>>) -> bool {
        if root == sub_root {
            return true;
        }
        if let Some(node) = root {
            let mut node = node.borrow_mut();

            Self::is(node.left.take().as_ref(), sub_root) ||
                Self::is(node.right.take().as_ref(), sub_root)
        } else {
            false
        }
    }
}