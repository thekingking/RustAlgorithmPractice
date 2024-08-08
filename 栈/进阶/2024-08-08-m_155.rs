struct MinStack {
    stack: Vec<i32>,
    min_pre: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        Self {
            stack: Vec::new(),
            min_pre: vec![i32::MAX],
        }
    }
    
    fn push(&mut self, val: i32) {
        self.min_pre.push(std::cmp::min(val, *self.min_pre.last().unwrap()));
        self.stack.push(val);
    }
    
    fn pop(&mut self) {
        self.min_pre.pop();
        self.stack.pop();
    }
    
    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }
    
    fn get_min(&self) -> i32 {
        *self.min_pre.last().unwrap()
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