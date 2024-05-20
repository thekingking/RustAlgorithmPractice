use std::collections::HashMap;
fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn longest_awesome(s: String) -> i32 {
        let mut ans = 1;
        let mut map = HashMap::new();
        map.insert(0, -1);
        let mut prev = 0;
        let mut lib = [0; 11];
        for i in 0..10 {
            lib[i + 1] = 1 << i;
        }
        for (idx, c) in s.bytes().enumerate() {
            prev ^= 1 << (c - 48);
            for target in lib {
                let cur = target ^ prev;
                if map.get(&cur).is_some() {
                    ans = ans.max(idx as i32 - map.get(&cur).unwrap());
                }
            }
            if map.get(&prev).is_none() {
                map.insert(prev, idx as i32);
            }
        }
        ans as i32
    }
}