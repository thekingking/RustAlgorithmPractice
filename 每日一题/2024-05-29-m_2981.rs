use std::collections::HashMap;

struct Solution;

impl Solution {
    // 写复杂了
    pub fn maximum_length(s: String) -> i32 {
        let mut cnt = HashMap::new();
        for (i, c) in s.chars().enumerate() {
            for (j, t) in s[0..=i].chars().rev().enumerate() {
                if c == t {
                    cnt.entry(&s[(i - j)..=i]).and_modify(|x| *x += 1).or_insert(1);
                } else {
                    break;
                }
            }
        }
        let mut max = -1;
        for (k, v) in cnt {
            if v >= 3 && k.len() as i32 > max {
                max = k.len() as i32;
            }
        }
        max
    }

    // 官解，一次遍历
    pub fn maximum_length(s: String) -> i32 {
        let mut ans = -1;
        let len = s.len();
        let mut chs = vec![vec![]; 26];
        let mut cnt = 0;
        let s_bytes = s.as_bytes();

        for i in 0..len {
            cnt += 1;
            if i + 1 == len || s_bytes[i] != s_bytes[i + 1] {
                let ch = (s_bytes[i] - b'a') as usize;
                chs[ch].push(cnt);
                cnt = 0;
                for j in (1..chs[ch].len()).rev() {
                    if chs[ch][j] > chs[ch][j - 1] {
                        chs[ch].swap(j, j - 1);
                    } else {
                        break;
                    }
                }
                if chs[ch].len() > 3 {
                    chs[ch].pop();
                }
            }
        }

        for i in 0..26 {
            if chs[i].len() > 0 && chs[i][0] > 2 {
                ans = ans.max(chs[i][0] - 2);
            }
            if chs[i].len() > 1 && chs[i][0] > 1 {
                ans = ans.max((chs[i][0] - 1).min(chs[i][1]));
            }
            if chs[i].len() > 2 {
                ans = ans.max(chs[i][2]);
            }
        }
        ans
    }
}