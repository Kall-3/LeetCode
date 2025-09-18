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
use std::collections::BinaryHeap;
use std::cmp::Reverse;

type Node = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn kth_smallest(root: Option<Node>, k: i32) -> i32 {
        let mut bh = BinaryHeap::new();
        Self::mo(&root, &mut bh);

        for _ in 1..k { bh.pop(); }
        bh.peek().unwrap().0
    }

    fn mo(root: &Option<Node>, bh: &mut BinaryHeap<Reverse<i32>>) {
        if let Some(rc) = root {
            bh.push(Reverse(rc.borrow().val));

            Self::mo(&rc.borrow().left, bh);
            Self::mo(&rc.borrow().right, bh);
        }
    }
}