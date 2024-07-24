struct Solution;

impl Solution {
    pub fn relocate_marbles(nums: Vec<i32>, move_from: Vec<i32>, move_to: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        
        let mut cnt: HashSet<i32> = nums.into_iter().collect();
        for i in 0..move_from.len() {
            if cnt.contains(&move_from[i]) {
                cnt.remove(&move_from[i]);
                cnt.insert(move_to[i]);
            }
        }
        let mut res: Vec<i32> = cnt.into_iter().collect();
        res.sort_unstable();
        res
    }
}
