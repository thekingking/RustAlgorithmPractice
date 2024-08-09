use std::collections::HashMap;

struct FreqStack {
    map: HashMap<i32, i32>,
    cnt: Vec<Vec<i32>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {

    fn new() -> Self {
        Self {
            map: HashMap::new(),
            cnt: Vec::new(),
        }
    }
    
    fn push(&mut self, val: i32) {
        let num = *self.map.get(&val).unwrap_or(&0) + 1;
        self.map.insert(val, num);
        let n = self.cnt.len();
        if (n as i32) < num {
            self.cnt.push(Vec::new());
        }
        self.cnt[num as usize - 1].push(val);
    }
    
    fn pop(&mut self) -> i32 {
        let n = self.cnt.len();
        let val = self.cnt[n - 1].pop().unwrap();
        if self.cnt[n - 1].is_empty() {
            self.cnt.pop();
        }
        self.map.entry(val).and_modify(|x| *x -= 1);
        val
    }
}



/**
 * Your FreqStack object will be instantiated and called as such:
 * let obj = FreqStack::new();
 * obj.push(val);
 * let ret_2: i32 = obj.pop();
 */