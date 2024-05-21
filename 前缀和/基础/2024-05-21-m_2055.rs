struct Solution;

impl Solution {
    // 我自己写的方法
    pub fn plates_between_candles1(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut left = vec![-1; s.len()];
        let mut right = vec![0; s.len()];
        let mut num = 0;
        for (idx, c) in s.chars().enumerate() {
            if c == '*' {
                num += 1;
            } else {
                left[idx] = num;
                right[idx] = num;
            }
        }
        num = 0;
        for i in 0..right.len() {
            if right[i] != 0 {
                num = right[i];
            }
            right[i] = num;
        }
        num = i32::MAX;
        for i in (0..left.len()).rev() {
            if left[i] != -1 {
                num = left[i];
            }
            left[i] = num;
        }
        let mut ans = vec![0; queries.len()];
        for (idx, q) in queries.iter().enumerate() {
            let mut temp = right[q[1] as usize] - left[q[0] as usize];
            if temp < 0 {
                temp = 0;
            }
            ans[idx] = temp;
        }
        ans
    }

    // 官解
    pub fn plates_between_candles2(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = s.len();
        let mut preSum = vec![0; n];
        let mut sum = 0;
        for (i, c) in s.chars().enumerate() {
            if c == '*' {
                sum += 1;
            }
            preSum[i] = sum;
        }
        let mut left = vec![0; n];
        let mut l = -1;
        for (i, c) in s.chars().enumerate() {
            if c == '|' {
                l = i as i32;
            }
            left[i] = l;
        }
        let mut right = vec![0; n];
        let mut r = -1;
        for (i, c) in s.chars().rev().enumerate() {
            if c == '|' {
                r = (n - i - 1) as i32;
            }
            right[n - i - 1] = r;
        }
        let mut ans = vec![0; queries.len()];
        for (i, q) in queries.iter().enumerate() {
            let x = right[q[0] as usize];
            let y = left[q[1] as usize];
            ans[i] = if x == -1 || y == -1 || x >= y { 0 } else {preSum[y as usize] - preSum[x as usize]};
        }
        ans
    }

    // 题解里面看到的，官解中的preSum,left,right的三合一求法
    pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (s.len(), queries.len());
        let s_chars = s.chars().collect::<Vec<_>>();
        let arr = (0..m).fold((vec![0; m + 1], vec![i32::MAX; m + 1], vec![0; m + 1]), |(mut prev, mut next, mut pre_sum), i| {
            pre_sum[i + 1] = if s_chars[i] == '|' { pre_sum[i] + 1 } else { pre_sum[i] };
            prev[i + 1] = if s_chars[i] == '|' { i as i32 } else { prev[i] };
            next[m - i - 1] = if s_chars[m - i - 1] == '|' { (m - i - 1) as i32 } else { next[m - i] };
            (prev, next, pre_sum)
        });

        queries.iter().fold(Vec::new(), |mut ret, query| {
            let (l, r) = (arr.1[query[0] as usize], arr.0[query[1] as usize + 1]);
            ret.push(if l < r { r - l - (arr.2[r as usize] - arr.2[l as usize])} else { 0 });
            ret
        })
    }
}
