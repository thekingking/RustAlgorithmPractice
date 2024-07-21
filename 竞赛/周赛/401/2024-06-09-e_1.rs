struct Solution;

impl Solution {
    pub fn number_of_child(n: i32, k: i32) -> i32 {
        let num = k % (2 * (n - 1));
        if num > n - 1 { 2 * n - 2 - num } else { num }
    }
}
