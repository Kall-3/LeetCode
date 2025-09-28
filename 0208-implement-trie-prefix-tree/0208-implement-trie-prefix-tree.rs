use std::boxed::Box;

struct Node {
    children: [Option<Box<Node>>; 26],
    is_end: bool,
}

impl Node {
    fn new() -> Self {
        Node {
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
            root: Node::new(),
        }
    }
    
    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;

        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            let slot = &mut node.children[idx];
            node = slot.get_or_insert(Box::new(Node::new())).as_mut();
        }

        node.is_end = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut node = &self.root;

        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            match &node.children[idx] {
                Some(next) => node = next,
                None => return false,
            }
        }

        node.is_end
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;

        for b in prefix.bytes() {
            let idx = (b - b'a') as usize;
            match &node.children[idx] {
                Some(next) => node = next,
                None => return false,
            }
        }

        true
    }
}