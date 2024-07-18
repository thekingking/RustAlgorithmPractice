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
    pub fn minimum_time(n: i32, edges: Vec<Vec<i32>>, disappear: Vec<i32>) -> Vec<i32> {
        let n = n as usize;
        let mut cnt = vec![vec![]; n];
        for edge in edges {
            let x = edge[0] as usize;
            let y = edge[1] as usize;
            let w = edge[2];
            cnt[x].push((y, -w));
            cnt[y].push((x, -w));
        }
        let mut res = vec![-1; n];
        let mut h = std::collections::BinaryHeap::new();
        h.push((0, 0));
        while let Some((x, i)) = h.pop() {
            if res[i] == -1 {
                res[i] = -x;
                for &(j, y) in &cnt[i] {
                    let dis = x + y;
                    if res[j] != -1 || -dis >= disappear[j] {
                        continue;
                    }
                    h.push((dis, j));
                }
            }
        }
        res
    }
}
