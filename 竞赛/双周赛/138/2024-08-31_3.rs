impl Solution {
    pub fn count_good_integers(n: i32, k: i32) -> i64 {
        use std::collections::HashSet;
        let mut factorial = vec![0; n as usize + 1];
        factorial[0] = 1;
        for i in 1..=n {
            factorial[i as usize] = factorial[i as usize - 1] * i as i64;
        }
        let mut ans = 0;
        let mut set = HashSet::new();
        let base = 10_i32.pow(((n - 1) / 2) as u32);
        for i in base..10 * base {
            let mut s = i.to_string();
            let mut bs = s.clone().into_bytes();
            bs.reverse();
            if n % 2 == 1 {
                bs = bs[1..].to_vec();
            }
            s.push_str(&unsafe {
                String::from_utf8_unchecked(bs)
            });
            let s_clone = s.clone();
            let num = s.parse::<i64>().unwrap();
            if num % k as i64 != 0 {
                continue;
            }
            let mut sort_s = s.into_bytes();
            sort_s.sort_unstable();
            let ss = unsafe {
                String::from_utf8_unchecked(sort_s)
            };

            if set.contains(&ss) {
                continue;
            } else {
                set.insert(ss);
            }
            let mut cnt = vec![0; 10];
            for &c in s_clone.as_bytes() {
                cnt[(c - b'0') as usize] += 1;
            }
            let mut res = (n - cnt[0]) as i64 * factorial[n as usize - 1];
            for c in cnt {
                res /= factorial[c as usize];
            }
            ans += res;
        }
        ans
    }
}