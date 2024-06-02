use std::collections::HashSet;

fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        candy_type.iter().collect::<HashSet<_>>().len().min(candy_type.len() / 2) as _
    }
}