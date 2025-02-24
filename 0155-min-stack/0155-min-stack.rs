struct MinStack {
    stack:  Vec<i32>,
    min:    Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        MinStack {
            stack:  Vec::new(),
            min:    Vec::new(),
        }
    }
    
    fn push(&mut self, val: i32) {
        self.stack.push(val);
        if val <= self.get_min() {
            self.min.push(val);
        }
    }
    
    fn pop(&mut self) {
        if self.stack.pop() == self.min.last().copied() {
            self.min.pop();
        }
    }
    
    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }
    
    fn get_min(&self) -> i32 {
        *self.min.last().unwrap_or(&i32::MAX)
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */