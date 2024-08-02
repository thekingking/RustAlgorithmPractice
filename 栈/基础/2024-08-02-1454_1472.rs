struct BrowserHistory {
    stack: Vec<String>,
    index: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BrowserHistory {

    fn new(homepage: String) -> Self {
        Self {
            stack: vec![homepage],
            index: 1,
        }
    }
    
    fn visit(&mut self, url: String) {
        while self.stack.len() > self.index {
            self.stack.pop();
        }
        self.stack.push(url);
        self.index += 1;
    }
    
    fn back(&mut self, steps: i32) -> String {
        self.index = self.index.saturating_sub(steps as usize).max(1);
        self.stack[self.index - 1].clone()
    }
    
    fn forward(&mut self, steps: i32) -> String {
        self.index = self.stack.len().min(self.index + steps as usize);
        self.stack[self.index - 1].clone()
    }
}

/**
 * Your BrowserHistory object will be instantiated and called as such:
 * let obj = BrowserHistory::new(homepage);
 * obj.visit(url);
 * let ret_2: String = obj.back(steps);
 * let ret_3: String = obj.forward(steps);
 */