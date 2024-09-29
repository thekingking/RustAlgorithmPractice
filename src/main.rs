use std::str::FromStr;


fn main() {
    println!("hello, world");
}

struct Solution;

impl Solution {
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
            let mut j = 1;
            while b == 0 && other == k && r + j < n && cnt[(bs[r] - b'a') as usize] >= 0 {
                j += 1;
            }
            res += j as i32;
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

    pub fn kth_character(k: i64, operations: Vec<i32>) -> char {
        let mut n = k - 1;
        let mut num = 0;
        for x in operations {
            if n % 2 == 1 && x == 1 {
                num += 1;
            }
            n /= 2;
        }
        (b'a' + (num % 26) as u8) as _
    }
}
