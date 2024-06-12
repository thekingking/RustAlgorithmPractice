struct Solution;

impl Solution {
    /// 简单题，四舍五入
    pub fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
        100 - (purchase_amount + 5) / 10 * 10
    }
}
