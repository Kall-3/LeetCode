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
        Solution::ist1(root, sub_root.as_ref())
    }

    fn ist1(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<&Rc<RefCell<TreeNode>>>) -> bool {
        if Solution::ist2(root.as_ref(), sub_root) {
            true
        } else {
            if let Some(root) = root {
                let mut root = root.borrow_mut();

                Solution::ist1(root.left.take(), sub_root) || Solution::ist1(root.right.take(), sub_root)
            } else {
                false
            }
        }
    }

    fn ist2(root: Option<&Rc<RefCell<TreeNode>>>, sub_root: Option<&Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            if let Some(sub) = sub_root {
                let sub = sub.borrow();
                let root = root.borrow();

                if root.val == sub.val {
                    return Solution::ist2(root.left.as_ref(), sub.left.as_ref())
                        && Solution::ist2(root.right.as_ref(), sub.right.as_ref());
                }
            }
        }

        if root.is_none() && sub_root.is_none() {
            true
        } else {
            false
        }
    }
}