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
    pub fn number_of_sets(n: i32, max_distance: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut cnt = vec![vec![i32::MAX / 2; n + 1]; n + 1];

        for road in roads {
            let x = road[0] as usize;
            let y = road[1] as usize;
            cnt[x][y] = std::cmp::min(cnt[x][y], road[2]);
            cnt[y][x] = std::cmp::min(cnt[y][x], road[2]);
        }

        fn check(cnt: &Vec<Vec<i32>>, s: i32, n: usize, max_distance: i32) -> i32 {
            let mut f = vec![vec![0; n + 1]; n + 1];
            for i in 0..n {
                if (s >> i) & 1 != 0 {
                    f[i] = cnt[i].clone();
                }
            }

            for k in 0..n {
                if (s >> k) & 1 == 0 {
                    continue;
                }
                for i in 0..n {
                    if (s >> i) & 1 == 0 {
                        continue;
                    }
                    for j in 0..n {
                        f[i][j] = std::cmp::min(f[i][j], f[i][k] + f[k][j]);
                    }
                }
            }

            for i in 0..n {
                if (s >> i) & 1 == 0 {
                    continue;
                }
                for j in 0..i {
                    if (s >> j) & 1 != 0 && f[i][j] > max_distance {
                        return 0;
                    }
                }
            }
            1
        }

        let mut ans = 0;
        for s in 0..(1 << n) {
            ans += check(&cnt, s, n, max_distance);
        }
        ans
    }
}
