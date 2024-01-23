use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;


impl Solution
{
    pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>)
    -> bool
    {
        // Depth of sub-tree
        let root_depth = Solution::max_depth(sub_root.clone());

        // Create new tree containing max-depths
        let mut depths = HashMap::new();

        // Calculate max-depths
        Solution::max_depth_save(root.clone(), &mut depths);
        println!("sub_root d: {}", root_depth);

        // Find sub-tree
        Solution::is_subtree_rec(root, &sub_root, &depths, root_depth)
    }

    // Find if depth matches, then check sub-tree
    fn is_subtree_rec
    (
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: &Option<Rc<RefCell<TreeNode>>>,
        depths: &HashMap<*const RefCell<TreeNode>, i32>,
        root_depth: i32
    ) -> bool
    {
        if let Some(node) = root
        {
            if let Some(depth) = depths.get(&Rc::as_ptr(&node))
            {
                if depth == &root_depth
                {
                    println!("depths match!");
                    if Solution::check_same(Some(node.clone()), sub_root)
                    {
                        return true
                    }
                }

                let temp = Solution::is_subtree_rec(node.borrow_mut().left.take(), &sub_root, &depths, root_depth);
                if temp { return true }
                return temp || Solution::is_subtree_rec(node.borrow_mut().right.take(), &sub_root, &depths, root_depth);
            }
        }
        false
    }

    fn check_same(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool
    {
        println!("Match start: {}, {}", root.is_some(), sub_root.is_some());
        match (root, sub_root) {
            (Some(root_node), Some(sub_node)) => {
                if root_node.borrow().val == sub_node.borrow().val
                {
                    println!("r/s: {}, {}", root_node.borrow().val, sub_node.borrow().val);
                    return Solution::check_same(root_node.borrow().left.clone(), &sub_node.borrow().left.clone()) && Solution::check_same(root_node.borrow().right.clone(), &sub_node.borrow().right.clone());
                }
            }
            (None, None) => {
                return true
            }
            _ => {
                return false
            }
        }
        false
    }

    // Save all depths from all nodes
    fn max_depth_save
    (
        root: Option<Rc<RefCell<TreeNode>>>,
        depths: &mut HashMap<*const RefCell<TreeNode>, i32>
    ) -> i32
    {
        if let Some(node) = root
        {
            // Calculate max-depth
            let left    = Solution::max_depth_save(node.borrow().left.clone(), depths);
            let right   = Solution::max_depth_save(node.borrow().right.clone(), depths);
            let max     = 1 + left.max(right);

            // Save depth
            // println!("d: {}, v: {}", max, node.borrow().val);
            depths.insert(Rc::as_ptr(&node), max);
            max
        }
        else
        {
            0
        }
    } 

    // Find all depths
    fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32
    {
        if let Some(ref node) = root
        {
            let mut ref_node    = node.borrow_mut();
            let right           = Solution::max_depth(ref_node.right.clone()) + 1;
            let left            = Solution::max_depth(ref_node.left.clone()) + 1;
            right.max(left)
        }
        else
        {
            0
        }
    } 
}