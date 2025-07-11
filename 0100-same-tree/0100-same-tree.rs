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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(p) = p {
            if let Some(q) = q {
                let mut p = p.borrow_mut();
                let mut q = q.borrow_mut();

                if p.val != q.val {
                    return false;
                }

                return Self::is_same_tree(p.left.take(), q.left.take())
                    && Self::is_same_tree(p.right.take(), q.right.take());
            } else {
                return false;
            }
        } else if !q.is_none() {
            return false;
        }
        true
    }
}