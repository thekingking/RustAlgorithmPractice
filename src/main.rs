use std::collections::HashMap;

fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len() as i32;
        let mut xor_all = 0;
        for row in &grid {
            for &x in row {
                xor_all ^= x;
            }
        }
        xor_all ^= if n % 2 == 1 { 1 } else { n * n };

        let mut ans = vec![0, 0];
        let low_bit = xor_all & -xor_all;
        for x in 1..=(n * n) {
            if x & low_bit != 0 {
                ans[0] ^= x;
            } else {
                ans[1] ^= x;
            }
        }
        for row in &grid {
            for &x in row {
                if x & low_bit != 0 {
                    ans[0] ^= x;
                } else {
                    ans[1] ^= x;
                }
            }
        }

        for row in grid {
            if row.contains(&ans[0]) {
                return ans
            }
        }
        vec![ans[1], ans[0]]
    }
}