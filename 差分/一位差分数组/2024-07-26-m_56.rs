struct Solution;

impl Solution {
    /// 排序
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::cmp::Ordering;

        intervals.sort_unstable_by(|a, b| {
            match (a, b) {
                (a, b) if a[0] > b[0] || (a[0] == b[0] && a[1] > b[1]) => Ordering::Greater,
                (a, b) if a[0] < b[0] || (a[0] == b[0] && a[1] < b[1]) => Ordering::Less,
                _ => Ordering::Equal
            }
        });
        let mut res = vec![];
        let mut start = intervals[0][0];
        let mut end = intervals[0][1];
        for row in &intervals[1..] {
            if row[0] <= end {
                end = std::cmp::max(end, row[1]);
            } else {
                res.push(vec![start, end]);
                start = row[0];
                end = row[1];
            }
        }
        res.push(vec![start, end]);
        res
    }

    /// 差分
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut cnt = [0; 20002];
        for row in intervals {
            cnt[row[0] as usize * 2] += 1;
            cnt[row[1] as usize * 2 + 1] -= 1;
        }
        let mut res = vec![];
        let mut sum = 0;
        let mut pre = 0;
        for (i, &x) in cnt.iter().enumerate() {
            if sum == 0 {
                pre = i;
            }
            if sum != 0 && sum + x == 0 {
                res.push(vec![pre as i32 / 2, i as i32 / 2]);
            }
            sum += x;
        }
        res
    }
}
