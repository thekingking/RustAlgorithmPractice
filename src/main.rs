use std::collections::HashMap;


fn main() {
    println!("hello, world");
}

struct Solution;

impl Solution {
    pub fn minimum_added_integer(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
        nums1.sort_unstable();
        nums2.sort_unstable();
        for i in (1..3).rev() {
            let x = nums2[0] - nums1[i];
            let mut j = 0;
            for &v in &nums1[i..] {
                if nums2[j] == v + x && { j += 1; j == nums2.len() } {
                    return x;
                }
            }
        }
        nums2[0] - nums1[0]
    }
}

struct DinnerPlates {
    capacity: usize,
    stacks: Vec<Vec<i32>>,
    cnt: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DinnerPlates {

    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            stacks: Vec::new(),
            cnt: 0,
        }
    }
    
    fn push(&mut self, val: i32) {
        if self.cnt == self.stacks.len() {
            self.stacks.push(vec![val]);
        } else {
            self.stacks[self.cnt].push(val);
        }
        if self.stacks[self.cnt].len() == self.capacity {
            while self.cnt < self.stacks.len() && self.stacks[self.cnt].len() == self.capacity {
                self.cnt += 1;
            }
        }
    }
    
    fn pop(&mut self) -> i32 {
        while let Some(stack) = self.stacks.last() {
            if stack.is_empty() {
                self.stacks.pop();
            } else {
                break;
            }
        }
        let n = self.stacks.len();
        if n == 0 {
            return -1;
        }
        let x = self.stacks[n - 1].pop().unwrap();
        self.cnt = self.cnt.min(self.stacks.len() - 1);
        x
    }
    
    fn pop_at_stack(&mut self, index: i32) -> i32 {
        let index = index as usize;
        if index >= self.stacks.len() {
            return -1;
        }
        let x = self.stacks[index].pop().unwrap_or(-1);
        if index < self.cnt {
            self.cnt = index;
        }
        x
    }
}
