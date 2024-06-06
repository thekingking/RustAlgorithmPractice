use std::collections::HashMap;

fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut cnt = HashMap::new();
        let mut pre_sum = 0;
        let mut pre_index = -1;
        let mut ans = 0;
        for x in nums {
          cnt.entry(pre_sum).or_insert(pre_index);
          pre_sum += if x == 1 { 1 } else { -1 };
          pre_index += 1;
          ans = ans.max(pre_index - *cnt.get(&pre_sum).unwrap_or(&pre_index));
        }
        ans
    }
}
