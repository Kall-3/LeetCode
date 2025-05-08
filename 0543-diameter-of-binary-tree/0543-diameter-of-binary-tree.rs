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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = 0;
        Solution::dobt(root, &mut max);
        max
    }

    fn dobt(root: Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
        if let Some(root) = root {
            let left = Solution::dobt(root.borrow_mut().left.take(), max);
            let right = Solution::dobt(root.borrow_mut().right.take(), max);

            *max = *max.max(&mut (left + right));

            1 + left.max(right)
        } else {
            0
        }
    }
}