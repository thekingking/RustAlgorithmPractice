struct Solution;

impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let n = s.len();
        let mut right = 0;
        let mut left = 0;
        let mut pre = vec![0; n + 1];
        let mut cnt = vec![0; n];
        let mut sum = 0;
        let mut one = 0;
        let mut flag = true;
        let bs = s.as_bytes();
        while right < n {
            if flag && bs[right] == b'1' {
                one += 1;
            }
            if right + 1 - left - one as usize <= k as usize || one <= k {
                right += 1;
                flag = true;
            } else {
                sum += (right - left) as i64;
                cnt[left] = right;
                if bs[left] == b'1' {
                    one -= 1;
                }
                left += 1;
                pre[left] = sum;
                flag = false;
            }
        }
        while left < n {
            sum += (right - left) as i64;
            cnt[left] = right;
            left += 1;
            pre[left] = sum;
        }
        let mut res = Vec::new();
        println!("{cnt:?}");
        println!("{pre:?}");
        for q in queries {
            println!("-----");
            let l = q[0];
            let r = q[1];
            let target = r + 1;
            let mut b = l;
            let mut t = r;
            while b < t {
                let mid = (b + t) / 2;
                if cnt[mid as usize] > target as usize {
                    t = mid - 1;
                } else {
                    b = mid + 1;
                }
            }
            println!("{l}, {r}, {t}");
            if cnt[b as usize] > r as usize {
                b -= 1;
            }
            let n = (r - b) as i64;
            b += 1;
            res.push(pre[b as usize] - pre[l as usize] + n * (n + 1) / 2);
        }
        res
    }
}