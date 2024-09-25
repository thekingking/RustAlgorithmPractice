struct Solution;

impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        use std::collections::HashSet;

        let mut cnt = vec![HashSet::new(); 26];
        let mut res = 0;
        for mut idea in ideas {
            let b = idea.remove(0) as u8 - b'a';
            cnt[b as usize].insert(idea);
        }
        let mut memo = vec![vec![0; 26]; 26];

        for i in 0..26 {
            for j in 0..26 {
                memo[i][j] = (cnt[i].len() - cnt[i].intersection(&cnt[j]).count()) as i64;
            }
        }
        for i in 0..26 {
            for j in 0..26 {
                res += memo[i][j] * memo[j][i];
            }
        }
        res
    }
}