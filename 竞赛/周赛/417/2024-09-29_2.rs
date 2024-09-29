struct Solution;

impl Solution {
    /// 灵神题解，斥容原理
    pub fn count_of_substrings(word: String, k: i32) -> i32 {
        fn f(bs: &[u8], k: i32) -> i32 {
            const VOWEL: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];
            let mut cnt = vec![0; 26];
            let mut num = 0;
            let mut b = (1 << (b'a' - b'a')) | (1 << (b'e' - b'a')) | (1 << (b'i' - b'a')) | (1 << (b'o' - b'a')) | (1 << (b'u' - b'a'));
            let mut left = 0;
            let mut ans = 0;
            for &c in bs {
                if VOWEL.contains(&c) {
                    if cnt[(c - b'a') as usize] == 0 {
                        b ^= 1 << (c - b'a');
                    }
                    cnt[(c - b'a') as usize] += 1;
                } else {
                    num += 1;
                }
                while b == 0 && num >= k {
                    if VOWEL.contains(&bs[left]) {
                        cnt[(bs[left] - b'a') as usize] -= 1;
                        if cnt[(bs[left] - b'a') as usize] == 0 {
                            b |= 1 << (bs[left] - b'a');
                        }
                    } else {
                        num -= 1;
                    }
                    left += 1;
                }
                ans += left as i32;
            }
            ans
        }
        let bs = word.as_bytes();
        f(bs, k) - f(bs, k + 1)
    }

    pub fn count_of_substrings(word: String, k: i32) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let mut res = 0;
        let bs = word.as_bytes();
        let n = bs.len();
        let mut cnt: Vec<i32> = vec![-1; 26];
        cnt[(b'a' - b'a') as usize] = 0;
        cnt[(b'e' - b'a') as usize] = 0;
        cnt[(b'i' - b'a') as usize] = 0;
        cnt[(b'o' - b'a') as usize] = 0;
        cnt[(b'u' - b'a') as usize] = 0;
        let mut other = 0;
        let mut b = (1 << (b'a' - b'a')) | (1 << (b'e' - b'a')) | (1 << (b'i' - b'a')) | (1 << (b'o' - b'a')) | (1 << (b'u' - b'a'));
        while r < n || l < n {
            while r < n && (b != 0 || other < k) {
                if cnt[(bs[r] - b'a') as usize] >= 0 {
                    if cnt[(bs[r] - b'a') as usize] == 0 {
                        b ^= 1 << bs[r] - b'a';
                    }
                    cnt[(bs[r] - b'a') as usize] += 1;
                } else {
                    other += 1;
                }
                r += 1;
            }
            let mut j = 0;
            while b == 0 && other == k && r + j < n && cnt[(bs[r + j] - b'a') as usize] >= 0 {
                j += 1;
            }
            if b == 0 && other == k {
                res += (j + 1) as i32;
            }
            if cnt[(bs[l] - b'a') as usize] >= 0 {
                cnt[(bs[l] - b'a') as usize] -= 1;
                if cnt[(bs[l] - b'a') as usize] == 0 {
                    b |= 1 << (bs[l] - b'a');
                }
            } else {
                other -= 1;
            }
            l += 1;
        }
        res
    }
}