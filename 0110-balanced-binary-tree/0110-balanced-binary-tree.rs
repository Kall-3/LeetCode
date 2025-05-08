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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::depth(root.as_ref()) != -1
    }

    fn depth(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node_rc) = root {
            let left = Solution::depth(node_rc.borrow().left.as_ref());
            let right = Solution::depth(node_rc.borrow().right.as_ref());

            if left == -1 || right == -1 {
                -1
            } else if (left - right).abs() < 2 {
                1 + left.max(right)
            } else {
                -1
            }
        } else {
            0
        }
    }
}