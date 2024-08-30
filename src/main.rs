use std::i32;


fn main() {
    println!("hello, world");
}

struct Solution;

impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, mut k: i32) -> i32 {
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::from(piles);
        while let Some(x) = heap.pop() {
            heap.push((x + 1) / 2);
            k -= 1;
            if k <= 0 {
                break;
            }
        }
        heap.into_iter().sum()
    }
}

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