struct SmallestInfiniteSet {
    cnt: Vec<bool>,
    min: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {

    fn new() -> Self {
        Self {
            cnt: vec![true; 1000],
            min: 0
        }
    }
    
    fn pop_smallest(&mut self) -> i32 {
        self.cnt[self.min] = false;
        let res = self.min as i32 + 1;
        while self.min < 1000 && !self.cnt[self.min] {
            self.min += 1;
        }
        res
    }
    
    fn add_back(&mut self, num: i32) {
        self.cnt[num as usize - 1] = true;
        self.min = self.min.min(num as usize - 1);
    }
}

/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */