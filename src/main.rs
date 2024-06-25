use std::collections::HashMap;

fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn good_subsetof_binary_matrix(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = 1 << grid[0].len();
        let mut cnt = vec![-1; n];
        for (i , row) in grid.iter().enumerate() {
            let mut mask = 0;
            for (j, &x) in row.iter().enumerate() {
                mask |= x << j;
            }
            if mask == 0 {
                return  vec![i as i32];
            }
            cnt[mask as usize] = i as i32;
        }
        let u = n as i32 - 1;
        for (x, &i) in cnt.iter().enumerate() {
            if i < 0 {
                continue;
            }
            let c = u ^ x as i32;
            let mut y = c;
            while y > 0 {
                let j = cnt[y as usize];
                if j >= 0 {
                    return vec![i.min(j), i.max(j)];
                }
                y = (y - 1) & c;
            }
        }
        vec![]
    }
}