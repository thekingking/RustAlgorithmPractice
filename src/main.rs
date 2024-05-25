use std::collections::HashMap;
fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        let mut max = 0;
        let mut min = 0;
        for j in (index_difference as usize)..nums.len() {
            let i = j - index_difference as usize;
            if nums[min] > nums[i] {
                min = i;
            }
            if nums[j] - nums[min] >= value_difference {
                return vec![min as i32, j as i32];
            }
            if nums[max] < nums[i] {
                max = i;
            }
            if nums[max] - nums[j] >= value_difference {
                return vec![max as i32, j as i32];
            }
        }
        vec![-1, -1]
    }
}