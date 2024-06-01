use std::collections::HashMap;

fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i32 {
        let c2 = |n| if n > 1 { n * (n - 1) / 2 } else { 0 };
        c2(n + 2) - 3 * c2(n - limit + 1) + 3 * c2(n - 2 * limit) - c2(n - 3 * limit - 1)
    }
}