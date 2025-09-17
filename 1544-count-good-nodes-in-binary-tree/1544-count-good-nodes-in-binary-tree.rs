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
    pub fn good_nodes(root: Option<Node>) -> i32 {
        Self::gn(&root, i32::MIN)
    }

    fn gn(root: &Option<Node>, prev_max: i32) -> i32 {
        if let Some(root_rc) = root {
            if root_rc.borrow().val >= prev_max {
                return 1 + Self::gn(&root_rc.borrow().left, root_rc.borrow().val) + Self::gn(&root_rc.borrow().right, root_rc.borrow().val);
            } else {
                return Self::gn(&root_rc.borrow().left, prev_max) + Self::gn(&root_rc.borrow().right, prev_max);
            }
        }
        0
    }
}