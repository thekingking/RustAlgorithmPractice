struct Solution;

impl Solution {
    // 跟昨天的题一样，对时间复杂度要求更高了
    pub fn maximum_length(s: String) -> i32 {
        let mut chs = vec![vec![]; 26];
        let mut ans = -1;
        let s_bytes = s.as_bytes();
        let len = s_bytes.len();
        let mut num = 0;
        for i in 0..len {
            num += 1;
            if i + 1 == len || s_bytes[i] != s_bytes[i + 1] {
                let ch = (s_bytes[i] - b'a') as usize;
                chs[ch].push(num);
                num = 0;
                for j in (1..chs[ch].len()).rev() {
                    if chs[ch][j] > chs[ch][j - 1] {
                        chs[ch].swap(j, j - 1);
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