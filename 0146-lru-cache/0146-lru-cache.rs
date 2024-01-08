use std::collections::HashMap;

#[derive(Clone)]
struct Node {
    data:   i32,
    next:   Option<Box<Node>>,
    prev:   Option<*mut Node>,
}

struct LinkedList {
    head:   Option<Box<Node>>, // MRU
    tail:   Option<*mut Node>, // LRU
}

impl LinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    // Insert at head, MRU
    fn push_front(&mut self, value: i32) -> *mut Node {
        let mut new_node = Box::new( Node { data: value, next: None, prev: None } );
        let mut head = self.head.take();

        if let Some(ref mut head) = head {
            head.prev = Some(&mut *new_node as *mut Node);
        } else {
            // New list, first element
            self.tail = Some(&mut *new_node as *mut Node);
        }

        new_node.next = head;
        let last = &mut *new_node as *mut Node;
        self.head = Some(new_node);
        last
    }

    fn pop_back(&mut self) -> Option<i32> {
        if let Some(tail) = self.tail {
            unsafe {
                if let Some(prev) = (*tail).prev {
                    self.tail = Some(prev);
                    (*prev).next = None;
                } else {
                    self.head = None;
                }
                return Some((*tail).data)
            }
        }
        None
    }

    fn remove(&mut self, node: *mut Node) {
        let mut node = unsafe { &mut *node };

        match (node.prev.take(), node.next.take()) {
            // Patch ends together
            (Some(prev), Some(mut next)) => unsafe {
                next.prev = Some(prev);
                (*prev).next = Some(next);
            }
            // Tail
            (Some(prev), None) => {
                unsafe { (*prev).next = None; }
                self.tail = Some(prev);
            }
            // Head
            (None, Some(mut next)) => {
                next.prev = None;
                self.head = Some(next);
            }
            // Single node
            (None, None) => {
                self.head = None;
                self.tail = None;
            }
        }
    }

}

struct LRUCache {
    capacity:   usize,
    map:        HashMap<i32, (i32, *mut Node)>,
    list:       LinkedList,
}

impl LRUCache {

    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity:   capacity as usize,
            map:        HashMap::new(),
            list:       LinkedList::new(),
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {

        if let Some((value, node)) = self.map.remove(&key) {
            // Remove old priority
            self.list.remove(node);

            // Insert again as MRU
            let node = self.list.push_front(key);
            self.map.insert(key, (value, node));
            value
        } else {
            -1
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {

        // If entry exists, remove and add again later
        if let Some((_, node)) = self.map.remove(&key) {
            self.list.remove(node);
        }
        // If capacity is exceded, pop LRU
        else if self.map.len() == self.capacity {
            if let Some(key) = self.list.pop_back() {
                self.map.remove(&key);
            }
        }
        // Insert new entry at most recent place
        let node = self.list.push_front(key);
        self.map.insert(key, (value, node));

    }
}