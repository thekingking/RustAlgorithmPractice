struct Solution;

impl Solution {
    /// BTreeMap 构建 差分数组
    pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
        let mut cnt = std::collections::BTreeMap::new();
        for row in intervals {
            cnt.entry(row[0]).and_modify(|x| *x += 1).or_insert(1);
            cnt.entry(row[1] + 1).and_modify(|x| *x -= 1).or_insert(-1);
        }
        let mut res = 0;
        let mut sum = 0;
        for &v in cnt.values() {
            sum += v;
            res = res.max(sum);
        }
        res
    }
}
