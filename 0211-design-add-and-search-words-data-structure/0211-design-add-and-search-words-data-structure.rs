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
        Self::search_rec(&mut node, word.as_bytes())
    }

    fn search_rec(start: &Node, word: &[u8]) -> bool {
        let mut cur = start;
        for (i, &b) in word.iter().enumerate() {
            if b == b'.' {
                return cur.children
                    .iter()
                    .filter_map(|opt| opt.as_ref())
                    .any(|child| Self::search_rec(child, &word[i+1..]));
            }

            let idx = (b - b'a') as usize;
            match &cur.children[idx] {
                Some(next) => cur = next,
                None => return false,
            };
        }

        cur.is_end
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */