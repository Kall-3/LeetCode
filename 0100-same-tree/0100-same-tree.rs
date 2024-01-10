use std::rc::Rc;
use std::cell::RefCell;

impl Solution
{
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool
    {
        Solution::is_same_tree_rec(p, q)
    }

    fn is_same_tree_rec(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool
    {
        if let (Some(p_node), Some(q_node)) = (p.as_ref(), q.as_ref())
        {
                p_node.as_ref().borrow().val == q_node.as_ref().borrow().val
            && 
                Solution::is_same_tree_rec
                (
                    p_node.as_ref().borrow().left.clone(),
                    q_node.as_ref().borrow().left.clone()
                )
            &&
                Solution::is_same_tree_rec
                (
                    p_node.as_ref().borrow().right.clone(),
                    q_node.as_ref().borrow().right.clone()
                )
        }
        else if p.as_ref().is_none() && q.as_ref().is_none()
        {
            true
        }
        else
        {
            false
        }
    }
}