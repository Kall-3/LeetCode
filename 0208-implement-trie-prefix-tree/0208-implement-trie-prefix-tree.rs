use std::{rc::Rc, cell::RefCell};

type Node = Rc<RefCell<TreeNode>>;

struct TreeNode {
    children: [Option<Node>; 26],
    is_end: bool,
}

impl TreeNode {
    fn new() -> Self {
        TreeNode {
            children: std::array::from_fn(|_| None),
            is_end: false,
        }
    }
}

struct Trie {
    root: Node,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: Rc::new(RefCell::new(TreeNode::new())),
        }
    }
    
    fn insert(&mut self, word: String) {
        let mut node = self.root.clone();

        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            let next = {
                node.borrow_mut().children[idx].get_or_insert(Rc::new(RefCell::new(TreeNode::new()))).clone()
            };
            node = next;
        }

        node.borrow_mut().is_end = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut node = self.root.clone();
        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            let next = {
                match &node.borrow().children[idx] {
                    Some(next) => next.clone(),
                    None => { return false },
                }
            };
            node = next;
        }

        node.borrow().is_end
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let mut node = self.root.clone();
        for b in prefix.bytes() {
            let idx = (b - b'a') as usize;
            let next = {
                match &node.borrow().children[idx] {
                    Some(next) => next.clone(),
                    None => { return false },
                }
            };
            node = next;
        }

        true
    }
}