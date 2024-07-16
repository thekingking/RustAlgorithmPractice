struct Solution;

impl Solution {
    /// 差分
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut cnt = vec![0; 101];
        for log in logs {
            cnt[log[0] as usize - 1950] += 1;
            cnt[log[1] as usize - 1950] -= 1;
        }
        let mut max = 0;
        let mut num = 0;
        let mut res = 0;
        for (i, &x) in cnt.iter().enumerate() {
            num += x;
            if num > max {
                res = i as i32 + 1950;
                max = num;
            }
        }
        res
    }
}
