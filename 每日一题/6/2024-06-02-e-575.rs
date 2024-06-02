use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    /// 简单题
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for candy in candy_type.iter() {
            map.insert(candy, 1);
        }
        (candy_type.len() / 2).min(map.len()) as i32
    }

    /// 集合，一行代码
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        candy_type.iter().collect::<HashSet<_>>().len().min(candy_type.len() / 2) as _
    }
}