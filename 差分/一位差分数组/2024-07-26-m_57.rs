struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut i = 0;
        let n = intervals.len();
        while i < n && intervals[i][1] < new_interval[0] {
            res.push(intervals[i].clone());
            i += 1;
        }
        let mut j = i;
        while j < n && intervals[j][0] <= new_interval[1] {
            j += 1;
        }
        if i == j {
            res.push(new_interval)
        } else {
            res.push(vec![std::cmp::min(new_interval[0], intervals[i][0]), std::cmp::max(new_interval[1], intervals[j - 1][1])]);
        }
        while j < n {
            res.push(intervals[j].clone());
            j += 1;
        }
        res
    }
}
