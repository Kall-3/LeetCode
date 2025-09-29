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

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("children", &self.children)
            .field("is_end", &self.is_end)
            .finish()
    }
}

struct WordDictionary {
    root: Node,
}

impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            root: Node::new(),
        }
    }
    
    fn add_word(&mut self, word: String) {
        let mut node = &mut self.root;

        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            let slot = &mut node.children[idx];
            node = slot.get_or_insert(Box::new(Node::new()));
        }

        node.is_end = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut node = &self.root;
        Self::search_rec(&mut node, word)
    }

    fn search_rec(mut node: &Node, word: String) -> bool {
        for (i, b) in word.bytes().enumerate() {
            if (b - b'.') == 0 {
                // check all possible branches
                println!("For word: {word}");
                return node.children
                    .iter()
                    .filter_map(|opt| opt.as_ref())
                    .any(|mut child| {
                        // println!("  found child {}", (b'a' + j as u8) as char);
                        Self::search_rec(&mut child, word[(i+1)..].to_string())
                    });
            }

            let idx = (b - b'a') as usize;
            match &node.children[idx] {
                Some(next) => node = next,
                None => return false,
            };
        }

        node.is_end
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */