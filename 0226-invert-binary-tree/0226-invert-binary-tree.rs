use std::rc::Rc;
use std::cell::RefCell;
impl Solution
{
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>
    {
        if let Some(ref node) = root {
            let mut ref_node    = node.borrow_mut();
            let right           = ref_node.right.take();
            ref_node.right      = Solution::invert_tree(ref_node.left.take());
            ref_node.left       = Solution::invert_tree(right);
        }
        root
    }
}