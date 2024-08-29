use std::i32;


fn main() {
    println!("hello, world");
}

struct Solution;

impl Solution {
    pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
        for i in 1..grid[0].len() {
            if grid[0][i] == grid[0][i - 1] {
                return false;
            }
        }
        for g in &grid[1..] {
            if &grid[0] != g {
                return false
            }
        }
        true
    }
}


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