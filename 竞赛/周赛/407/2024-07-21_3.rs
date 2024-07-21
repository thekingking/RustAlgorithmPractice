struct Solution;

impl Solution {
    /// 灵神题解
    pub fn max_operations(s: String) -> i32 {
        let mut ans = 0;
        let mut cnt = 0;
        let bs = s.as_bytes();
        for i in 0..bs.len() {
            if bs[i] == b'1' {
                cnt += 1;
            } else if i > 0 && bs[i - 1] == b'1' {
                ans += cnt;
            }
        }
        ans
    }

    /// 周赛
    pub fn max_operations(s: String) -> i32 {
        let mut res = 0;
        let mut sum = 0;
        let bs = s.as_bytes();
        let mut i = 0;
        let mut flag = false;
        while i < bs.len() {
            if bs[i] == b'1' {
                if flag {
                    res += sum;
                    flag = false;
                }
                sum += 1;
            } else {
                flag = true;
            }
            i += 1;
        }
        if flag {
            res += sum;
        }
        res
    }
}
