use std::rc::Rc;
use std::cell::RefCell;

type Node = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn max_path_sum(root: Option<Node>) -> i32 {
        let mut max = i32::MIN;
        Self::mps(&root, &mut max);
        max
    }

    fn mps(node: &Option<Node>, max: &mut i32) -> i32 {
        if let Some(rc) = node {
            let val = rc.borrow().val;
            let left = Self::mps(&rc.borrow().left, max);
            let right = Self::mps(&rc.borrow().right, max);

            *max = (val + left + right)
                .max(val + left)
                .max(val + right)
                .max(val)
                .max(*max);

            val.max(val + right)
                .max(val + left)
        } else {
            0
        }
    }
}