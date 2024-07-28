struct Solution;

impl Solution {
    /// 暴力
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        let mut cnt: Vec<Vec<i32>> = Vec::new();
        let mut res = Vec::new();
        let mut max = 0;
        for p in &positions {
            let l = p[0];
            let r = p[0] + p[1];
            let mut h = 0;
            for c in &cnt {
                if l < c[1] && r > c[0] {
                    h = h.max(c[2]);
                }
            }
            cnt.push(vec![l, r, h + p[1]]);
            max = max.max(h + p[1]);
            res.push(max);
        }
        res
    }
}
