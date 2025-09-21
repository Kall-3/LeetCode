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
use std::collections::HashMap;

type Node = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Node> {
        let io_idx: HashMap<i32, usize> = inorder.iter().enumerate().map(|(i, &n)| (n, i)).collect();
        Self::build(&preorder, 0, preorder.len() - 1, 0, inorder.len() - 1, &io_idx)
    }

    fn build(po: &Vec<i32>, po_s: usize, po_e: usize, io_s: usize, io_e: usize, io_idx: &HashMap<i32, usize>) -> Option<Node> {
        if po_s > po_e {
            return None;
        }

        let val_to_add = po[po_s];
        let node = Rc::new(RefCell::new(TreeNode::new(val_to_add)));

        let io_split_idx = io_idx[&val_to_add];
        let io_left_s = io_s;
        let io_left_e = io_split_idx - 1;
        let io_right_s = io_split_idx + 1;
        let io_right_e = io_e;

        let po_left_s = po_s + 1;
        let po_left_e = po_left_s + (io_left_e - io_left_s);
        let po_right_s = po_left_e + 1;
        let po_right_e = po_e;

        node.borrow_mut().left = Self::build(&po, po_left_s, po_left_e, io_left_s, io_left_e, io_idx);
        node.borrow_mut().right = Self::build(&po, po_right_s, po_right_e, io_right_s, io_right_e, io_idx);

        Some(node)
    }
}