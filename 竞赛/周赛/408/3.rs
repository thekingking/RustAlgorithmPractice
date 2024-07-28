struct Solution;

impl Solution {
    /// 枚举
    pub fn number_of_substrings(s: String) -> i32 {
        let mut cnt = Vec::new();
        for (i, c) in s.chars().enumerate() {
            if c == '0' {
                cnt.push(i);
            }
        }
        let total1 = s.len() - cnt.len();
        cnt.push(s.len());
        let mut i = 0;
        let mut ans = 0;
        for (left, c) in s.chars().enumerate() {
            if c == '1' {
                ans += cnt[i] - left;
            }
            for k in i..cnt.len() - 1 {
                let cnt0 = k - i + 1;
                if cnt0 * cnt0 > total1 {
                    break;
                }
                let cnt1 = cnt[k] - left - (k - i);
                ans += (cnt[k + 1] - cnt[k]).saturating_sub((cnt0 * cnt0).saturating_sub(cnt1));
            }
            if c == '0' {
                i += 1;
            }
        }
        ans as i32
    }
}
