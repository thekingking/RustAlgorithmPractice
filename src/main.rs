use std::collections::HashMap;

fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn remove_trailing_zeros(num: String) -> String {
        num.trim_end_matches('0').to_string()
    }
}