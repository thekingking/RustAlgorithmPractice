use std::collections::HashMap;

fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut cnt = HashMap::new();
        let mut pre_sum = 0;
        let mut pre_index = -1;
        for x in nums {
            cnt.entry(pre_sum).or_insert(pre_index);
            pre_sum = (pre_sum + x) % k;
            pre_index += 1;
            if let Some(index) = cnt.get(&pre_sum) {
                if pre_index - index >= 2 {
                    return true;
                }
            }
        }
        false
    }
}