use std::rc::Rc;
use std::cell::RefCell;
impl Solution
{
    pub fn diameter_of_binary_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32
    {
        Solution::diameter_recursive(root).1
    }

    fn diameter_recursive(mut root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32)
    {
        if let Some(node) = root
        {
            let mut ref_node    = node.borrow_mut();
            let (left_depth, left_dia) = Solution::diameter_recursive(ref_node.left.take());
            let (right_depth, right_dia) = Solution::diameter_recursive(ref_node.right.take());

            (left_depth.max(right_depth) + 1, left_dia.max(right_dia.max(left_depth + right_depth)))
        }
        else
        {
            (0, 0)
        }
    }
}