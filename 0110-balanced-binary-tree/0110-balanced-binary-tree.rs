use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::is_balanced_recursive(root).1
    }

    fn is_balanced_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, bool)
    {
        if let Some(node) = root
        {
            let mut ref_node = node.borrow_mut();

            let left    = Solution::is_balanced_recursive(ref_node.left.take());
            let right   = Solution::is_balanced_recursive(ref_node.right.take());

            (left.0.max(right.0) + 1, ((left.0 - right.0).abs() < 2 && left.1 && right.1))
        }
        else
        {
            (0, true)
        }
    }
}