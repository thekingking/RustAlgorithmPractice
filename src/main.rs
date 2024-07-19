use std::{collections::{HashMap, HashSet}, vec};

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
    pub fn minimum_levels(possible: Vec<i32>) -> i32 {
        let mut sum = 0;
        for &p in &possible {
            if p == 1 {
                sum += 1;
            } else {
                sum -= 1;
            }
        }
        let mut res = 0;
        for (i, &p) in possible[0..(possible.len() - 1)].iter().enumerate() {
            if p == 1 {
                res += 1;
            } else {
                res -= 1;
            }
            if res * 2 > sum {
                return i as i32 + 1;
            }
        }
        -1
    }
}
