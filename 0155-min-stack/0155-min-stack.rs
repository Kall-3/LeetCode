struct MinStack {
    stack:      Vec<i32>,
    min_stack:  Vec<i32>,
}

impl MinStack {

    fn new() -> Self {
        Self {
            stack:      Vec::with_capacity(200),
            min_stack:  Vec::with_capacity(200),
        }
    }
    
    fn push(&mut self, val: i32) {
        self.stack.push(val);
        if val <= self.get_min() {
            self.min_stack.push(val);
        }
    }
    
    fn pop(&mut self) {
        if self.top() == self.get_min() {
            self.min_stack.pop();
        }
        self.stack.pop();
        
    }
    
    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }
    
    fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap_or(&i32::MAX)
    }
}