struct Solution;

impl Solution {
    pub fn edge_score(edges: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut cnt = HashMap::new();
        for (i, &x) in edges.iter().enumerate() {
            cnt.entry(x).and_modify(|x| *x += i as i64).or_insert(i as i64);
        }
        let mut res = 0;
        let mut max = 0;
        for (k, v) in cnt {
            if max < v || max == v && res > k {
                res = k;
                max = v;
            }
        }
        res
    }
}