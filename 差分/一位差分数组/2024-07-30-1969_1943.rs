struct Solution;

impl Solution {
    /// 差分
    pub fn split_painting(segments: Vec<Vec<i32>>) -> Vec<Vec<i64>> {
        let mut cnt = std::collections::BTreeMap::new();
        for seg in segments {
            cnt.entry(seg[0]).and_modify(|x| *x += seg[2] as i64).or_insert(seg[2] as i64);
            cnt.entry(seg[1]).and_modify(|x| *x -= seg[2] as i64).or_insert(-seg[2] as i64);
        }
        let mut res = Vec::new();
        let mut sum = 0;
        let mut pre = 0;
        for (k, v) in cnt {
            if sum != 0 {
                res.push(vec![pre as i64, k as i64, sum]);
            }
            pre = k;
            sum += v;
        }
        res
    }
}
