struct Solution;

impl Solution {
    pub fn clear_stars(s: String) -> String {
        let mut cnt = vec![vec![]; 26];
        let s_bytes = s.as_bytes();
        for i in 0..s_bytes.len() {
            if s_bytes[i] == b'*' {
                let mut j = 0;
                while cnt[j].len() == 0 {
                    j += 1;
                }
                cnt[j].pop();
            } else {
                cnt[(s_bytes[i] - b'a') as usize].push(i);
            }
        }
        let mut ans = vec![];
        for i in 0..26 {
            for j in 0..cnt[i].len() {
                ans.push(cnt[i][j]);
            }
        }
        ans.sort();
        String::from_iter(ans.iter().map(|x| (s_bytes[*x]) as char))
    }
}