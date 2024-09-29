struct Solution;

impl Solution {
    /// 斥容原理
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        fn f(bs: &[u8], k: i32) -> i64 {
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
                ans += left as i64;
            }
            ans
        }
        let bs = word.as_bytes();
        f(bs, k) - f(bs, k + 1)
    }
}
