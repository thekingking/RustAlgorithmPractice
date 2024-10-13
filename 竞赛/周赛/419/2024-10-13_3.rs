struct Solution;

impl Solution {
    pub fn count_winning_sequences(s: String) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let bs = s.as_bytes();
        let mut res = 0;
        let mut cnt = vec![vec![0; 2002]; 3];
        match bs[0] {
            b'F' => {
                cnt[0][1000] = 1;
                cnt[1][1000 + 1] = 1;
                cnt[2][1000 - 1] = 1;
            },
            b'W' => {
                cnt[0][1000 - 1] = 1;
                cnt[1][1000] = 1;
                cnt[2][1000 + 1] = 1;
            },
            b'E' => {
                cnt[0][1000 + 1] = 1;
                cnt[1][1000 - 1] = 1;
                cnt[2][1000] = 1;
            },
            _ => (),
        }
        for i in 1..bs.len() {
            let mut new_cnt = vec![vec![0; 2002]; 3];
            let mut arr = vec![0_i32; 3];
            match bs[i] {
                b'F' => {
                    arr[1] = 1;
                    arr[2] = -1;
                },
                b'W' => {
                    arr[0] = -1;
                    arr[2] = 1;
                },
                b'E' => {
                    arr[0] = 1;
                    arr[1] = -1;
                },
                _ => (),
            };

            for j in 500..2000 {
                for k in 0..3 {
                    new_cnt[k][(j + arr[k]) as usize] = (cnt[(k + 1) % 3][j as usize] + cnt[(k + 2) % 3][j as usize]) % MOD;
                }
            }
            cnt = new_cnt;
        }
        for i in 1001..2002 {
            res = (res + cnt[0][i] + cnt[1][i] + cnt[2][i]) % MOD;
        }
        res as i32
    }
}
