use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    key: i32,
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Rc<RefCell<Node>>>,
}

struct LRUCache {
    capacity: usize,
    lru: HashMap<i32, Rc<RefCell<Node>>>,
    head: Rc<RefCell<Node>>,
    tail: Rc<RefCell<Node>>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let lru = LRUCache {
            capacity: capacity as usize,
            lru: HashMap::new(),
            head: Rc::new(RefCell::new(Node {
                key: 0,
                val: 0,
                next: None,
                prev: None,
            })),
            tail: Rc::new(RefCell::new(Node {
                key: 0,
                val: 0,
                next: None,
                prev: None,
            })),
        };
        lru.head.borrow_mut().next = Some(lru.tail.clone());
        lru.tail.borrow_mut().prev = Some(lru.head.clone());
        lru
    }
    
    fn get(&mut self, key: i32) -> i32 {
        if let Some(node_rc) = self.lru.get(&key).cloned() {
            let val = node_rc.borrow().val;
            self.remove(node_rc.clone());
            self.insert_head(node_rc);
            return val;
        }
        -1
    }
    
    fn put(&mut self, key: i32, value: i32) {
        let new_rc = Rc::new(RefCell::new(Node {
            key: key,
            val: value,
            next: None,
            prev: None,
        }));

        if self.lru.contains_key(&key) {
            let old = self.lru.remove(&key);
            self.lru.insert(key, new_rc.clone());
            if let Some(old_rc) = old {
                self.remove(old_rc);
            }
        } else if self.lru.len() == self.capacity {
            let old = self.tail.borrow().prev.clone();
            if let Some(old_rc) = old {
                self.lru.remove(&old_rc.borrow().key);
                self.remove(old_rc.clone());
            }
        }

        self.lru.insert(key, new_rc.clone());
        self.insert_head(new_rc);
    }

    fn remove(&mut self, node_rc: Rc<RefCell<Node>>) {
        let next = node_rc.borrow().next.clone();
        let prev = node_rc.borrow().prev.clone();

        if let Some(ref next_rc) = next {
            next_rc.borrow_mut().prev = prev.clone();
        }

        if let Some(ref prev_rc) = prev {
            prev_rc.borrow_mut().next = next.clone();
        }

        node_rc.borrow_mut().next = None;
        node_rc.borrow_mut().prev = None;
    }

    fn insert_head(&mut self, node_rc: Rc<RefCell<Node>>) {
        let old = self.head.borrow().next.clone();

        if let Some(ref old_rc) = old {
            old_rc.borrow_mut().prev = Some(node_rc.clone());
        }
        self.head.borrow_mut().next = Some(node_rc.clone());

        node_rc.borrow_mut().next = old;
        node_rc.borrow_mut().prev = Some(self.head.clone());
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */