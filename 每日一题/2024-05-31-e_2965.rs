use std::collections::HashMap;

struct Solution;

impl Solution {
    /// 没想出来空间复杂度O(1)的解，我自己写的
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = (grid.len() as i32).pow(2);
        let mut map = HashMap::new();
        let mut sum = 0;
        let mut a = 0;
        for v in grid {
            for k in v {
                sum += k;
                if a == 0 {
                    map.entry(k).and_modify(|x| *x += 1).or_insert(1);
                    if *map.get(&k).unwrap() == 2 {
                        a = k;
                    }
                }
            }
        }
        vec![a, a + (n + 1) * n / 2 - sum]
    }

    /// 灵山大佬写的题解，用数学方法解题，666
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