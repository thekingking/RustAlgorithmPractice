use std::collections::HashMap;

fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len() as i32;
        let m = n * n;
        let mut d1 = -(m + 1) * m / 2;
        let m = m as i64;
        let mut d2 = -m * (m + 1) * (m * 2 + 1) / 6;
        for row in grid {
            for x in row {
                d1 += x;
                d2 += (x * x) as i64;
            }
        }
        let d = (d2 / d1 as i64) as i32;
        vec![(d + d1) / 2, (d - d1) / 2]
    }
}