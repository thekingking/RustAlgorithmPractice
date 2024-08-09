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
