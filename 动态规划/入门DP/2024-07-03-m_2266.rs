struct Solution;

impl Solution {
    pub fn count_texts(pressed_keys: String) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut k3 = Vec::new();
        let mut k4 = Vec::new();
        let bs = pressed_keys.as_bytes();
        let mut pre = bs[0];
        let mut sum = 0;
        let mut mk4 = 0;
        let mut mk3 = 0;
        for &b in bs {
            if b == pre {
                sum += 1;
            } else {
                if pre == b'7' || pre == b'9' {
                    if sum > mk4 {
                        mk4 = sum;
                    }
                    k4.push(sum);
                } else {
                    if sum > mk3 {
                        mk3 = sum;
                    }
                    k3.push(sum);
                }
                sum = 1;
            }
            pre = b;
        }
        if pre == b'7' || pre == b'9' {
            if sum > mk4 {
                mk4 = sum;
            }
            k4.push(sum);
        } else {
            if sum > mk3 {
                mk3 = sum;
            }
            k3.push(sum);
        }
        let mk3 = mk3 as usize;
        let mk4 = mk4 as usize;
        let mut dpk3 = vec![0; mk3 + 1];
        dpk3[0] = 1;
        let mut dpk4 = vec![0; mk4 + 1];
        dpk4[0] = 1;
        for i in 1..=mk3 {
            if i > 0 {
                dpk3[i] = (dpk3[i] + dpk3[i - 1]) % MOD;
            }
            if i > 1 {
                dpk3[i] = (dpk3[i] + dpk3[i - 2]) % MOD;
            }
            if i > 2 {
                dpk3[i] = (dpk3[i] + dpk3[i - 3]) % MOD;
            }
        }
        for i in 1..=mk4 {
            if i > 0 {
                dpk4[i] = (dpk4[i] + dpk4[i - 1]) % MOD;
            }
            if i > 1 {
                dpk4[i] = (dpk4[i] + dpk4[i - 2]) % MOD;
            }
            if i > 2 {
                dpk4[i] = (dpk4[i] + dpk4[i - 3]) % MOD;
            }
            if i > 3 {
                dpk4[i] = (dpk4[i] + dpk4[i - 4]) % MOD;
            }
        }
        let mut res = 1;
        for k in k3 {
            res = (res * dpk3[k as usize]) % MOD;
        }
        for k in k4 {
            res = (res * dpk4[k as usize]) % MOD;
        }
        res as i32
    }
}