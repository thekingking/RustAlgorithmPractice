use std::i32;


fn main() {
    println!("hello, world");
}

struct Solution;

impl Solution {
    pub fn minimum_substrings_in_partition(s: String) -> i32 {
        let mut dp = vec![0; s.len() + 1];
        let bs = s.as_bytes();
        let n = bs.len();
        for i in 1..=n {
            let mut max = 0;
            let mut res = i as i32;
            let mut cnt = vec![0; 26];
            for j in (0..i).rev() {
                let k = (bs[j] - b'a') as usize;
                cnt[k] += 1;
                if cnt[k] > max {
                    max = cnt[k];
                }
                if cnt.iter().all(|&x| x == max || x == 0) {
                    res = res.min(1 + dp[j]);
                }
            }
            dp[i] = res;
        }
        dp[bs.len()]
    }
}