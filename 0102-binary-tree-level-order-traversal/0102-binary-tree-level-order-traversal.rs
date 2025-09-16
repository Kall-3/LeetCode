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
type List = Vec<Vec<i32>>;

impl Solution {
    pub fn level_order(root: Option<Node>) -> List {
        let mut res: List = Vec::with_capacity(5);
        Self::lo(0, &root, &mut res);
        res.shrink_to_fit();
        res
    }

    fn lo(depth: usize, root: &Option<Node>, res: &mut List) {
        if let Some(root_rc) = root.as_ref() {
            if let Some(v) = res.get_mut(depth) { v.push(root_rc.borrow().val) }
            else { res.push(vec![root_rc.borrow().val]) }

            Self::lo(depth + 1, &root_rc.borrow().left, res); 
            Self::lo(depth + 1, &root_rc.borrow().right, res); 
        }
    }

}