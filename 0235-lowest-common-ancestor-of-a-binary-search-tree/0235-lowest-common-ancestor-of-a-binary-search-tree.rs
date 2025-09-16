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

type Node = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn lowest_common_ancestor(root: Option<Node>, p: Option<Node>, q: Option<Node>) -> Option<Node> {
        Self::lca(&root, &p.unwrap(), &q.unwrap())
    }

    fn lca(root: &Option<Node>, p: &Node, q: &Node) -> Option<Node> {
        let r = root.as_ref()?;
        if Rc::ptr_eq(r, p) || Rc::ptr_eq(r, q) {
            return Some(Rc::clone(r));
        }

        if let Some(root_rc) = root.as_ref() {
            let p_left = Self::exist_below(&root_rc.borrow().left, p);
            let q_left = Self::exist_below(&root_rc.borrow().left, q);

            if (p_left && q_left) {
                return Self::lca(&root_rc.borrow().left, p, q);
            } else if (!p_left && !q_left) {
                return Self::lca(&root_rc.borrow().right, p, q);
            }
        }
        Some(Rc::clone(r))
    }

    fn exist_below(root: &Option<Node>, n: &Node) -> bool {
        if let Some(root_rc) = root {
            if root_rc == n {
                return true;
            }

            return Self::exist_below(&root_rc.borrow().left, n) || Self::exist_below(&root_rc.borrow().right, n);
        }
        return false;
    }
}