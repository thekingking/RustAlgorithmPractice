struct Solution;

impl Solution {
    pub fn get_final_state(mut nums: Vec<i32>, mut k: i32, multiplier: i32) -> Vec<i32> {
        use std::collections::BinaryHeap;
        if multiplier == 1 {
            return nums;
        }
        fn pow(mut x: i64, mut n: i32) -> i64 {
            let mut res = 1;
            while n > 0 {
                if n % 2 == 1 {
                    res = res * x % MOD;
                }
                x = x * x % MOD;
                n /= 2;
            }
            res
        }
        const MOD: i64 = 1_000_000_007;
        let n = nums.len();
        let mx = *nums.iter().max().unwrap() as i64;
        let h: Vec<(i64, i32)> = nums.into_iter().enumerate().map(|(i, x)| (-x as i64, -(i as i32))).collect();
        let mut heap = BinaryHeap::from(h);

        while let Some(&(x, i)) = heap.peek() {
            if x > -mx && k > 0 {
                heap.pop();
                heap.push((x * multiplier as i64, i));
                k -= 1;
            } else {
                break;
            }
        }
        let mut cnt = heap.into_iter().map(|(x, i)| (-x, (-i) as usize)).collect::<Vec<(i64, usize)>>();
        cnt.sort();
        let mut res = vec![0; n];
        for i in 0..n {
            let (x, j) = cnt[i];
            res[j] = (x % MOD * pow(multiplier as i64, k / n as i32 + if i < k as usize % n { 1 } else { 0 }) % MOD) as i32;
        }
        res
    }
}