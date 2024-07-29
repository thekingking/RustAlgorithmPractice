struct Solution;

impl Solution {
    /// 栈
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut cnt = Vec::new();
        for s in operations {
            if let Ok(num) = s.parse::<i32>() {
                cnt.push(num);
            } else {
                let c = s.as_bytes()[0];
                if c == b'+' {
                    cnt.push(cnt[cnt.len() - 1] + cnt[cnt.len() - 2]);
                } else if c == b'D' {
                    cnt.push(cnt[cnt.len() - 1] * 2);
                } else {
                    cnt.pop();
                }
            }
        }
        cnt.iter().sum()
    }
}
