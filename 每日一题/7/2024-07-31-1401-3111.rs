struct Solution;

impl Solution {
    /// 贪心
    pub fn min_rectangles_to_cover_points(points: Vec<Vec<i32>>, w: i32) -> i32 {
        let mut cnt = std::collections::HashSet::new();
        for p in points {
            cnt.insert(p[0]);
        }
        let mut res = -w - 1;
        let mut cnt = cnt.into_iter().collect::<Vec<i32>>();
        cnt.sort_unstable();
        let mut pre = 0;
        for c in cnt {
            if c - pre > w {
                pre = c;
                res += 1;
            }
        }
        res
    }
}
