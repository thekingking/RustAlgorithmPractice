use std::collections::{HashMap, HashSet};

fn main() {
    println!("hello, world");
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut cnt1 = [false; 101];
        let mut cnt2 = [false; 101];
        let mut res1 = 0;
        let mut res2 = 0;
        for &x in &nums1 {
            cnt1[x as usize] = true;
        }
        for &x in &nums2 {
            cnt2[x as usize] = true;
        }
        for &x in &nums1 {
            res1 += cnt2[x as usize] as i32;
        }
        for &x in &nums2 {
            res2 += cnt1[x as usize] as i32;
        }
        vec![res1, res2]
    }
}
