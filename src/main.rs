use std::collections::HashMap;



fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
        100 - (purchase_amount + 5) / 10 * 10
    }
}
