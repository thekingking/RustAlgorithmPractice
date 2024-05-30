use std::collections::HashMap;

fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = HashMap::new();
        let mut ans = 0;
        let mut sum = 0;
        for i in 0..nums.len() {
            cnt.entry(sum).and_modify(|x| *x += 1).or_insert(1);
            sum += nums[i];
            ans += cnt.get(&(sum - k)).unwrap_or(&0);
        }
        ans
    }
}