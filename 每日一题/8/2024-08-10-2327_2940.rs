struct Solution;

impl Solution {
    /// 单调栈 + 记忆化搜索
    pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        // 单调栈
        let mut stack = vec![(i32::MAX, 0)];
        let mut cnt = vec![-1; heights.len()];
        for (i, &h) in heights.iter().enumerate() {
            while let Some(&(x, j)) = stack.last() {
                if h > x {
                    cnt[j] = i as i32;
                    stack.pop();
                } else {
                    stack.push((h, i));
                    break;
                }
            }
        }
        let mut res = vec![0; queries.len()];
        // 记忆化搜索
        let mut memo = std::collections::HashMap::new();
        for (i, q) in queries.iter().enumerate() {
            let a = std::cmp::min(q[0], q[1]) as usize;
            let b = std::cmp::max(q[0], q[1]) as usize;
            if let Some(&t) = memo.get(&(a, b)) {
                res[i] = t;
            } else {
                res[i] = if heights[a] < heights[b] || a == b {
                    b as i32
                } else {
                    let h = heights[a];
                    let mut t = b as i32;
                    while t != -1 && heights[t as usize] <= h {
                        t = cnt[t as usize];
                    }
                    t
                };
                memo.insert((a, b), res[i]);
            }
        }
        res
    }  
}
