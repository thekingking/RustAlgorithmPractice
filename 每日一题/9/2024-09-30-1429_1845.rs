struct SeatManager {
    arr: Vec<bool>,
    cur: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {

    fn new(n: i32) -> Self {
        Self {
            arr: vec![false; n as usize],
            cur: 0,
        }
    }
    
    fn reserve(&mut self) -> i32 {
        self.arr[self.cur] = true;
        let res = self.cur as i32 + 1;
        while self.cur < self.arr.len() && self.arr[self.cur] {
            self.cur += 1;
        }
        res
    }
    
    fn unreserve(&mut self, seat_number: i32) {
        let seat_number = seat_number as usize - 1;
        self.arr[seat_number] = false;
        if self.cur > seat_number {
            self.cur = seat_number;
        }
    }
}

/**
 * Your SeatManager object will be instantiated and called as such:
 * let obj = SeatManager::new(n);
 * let ret_1: i32 = obj.reserve();
 * obj.unreserve(seatNumber);
 */