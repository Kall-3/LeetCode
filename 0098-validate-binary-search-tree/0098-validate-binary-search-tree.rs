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

type Node = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn is_valid_bst(root: Option<Node>) -> bool {
        Self::ivb(&root, i64::MIN, i64::MAX)
    }

    fn ivb(root: &Option<Node>, last_max: i64, last_min: i64) -> bool {
        if let Some(root_rc) = root {
            let val = root_rc.borrow().val as i64;

            if val > last_max && val < last_min {
                Self::ivb(&root_rc.borrow().left, last_max, val) && Self::ivb(&root_rc.borrow().right, val, last_min)
            } else {
                false
            }
        } else {
            true
        }
    }
}