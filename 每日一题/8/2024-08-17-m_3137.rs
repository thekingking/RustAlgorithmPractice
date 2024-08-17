struct Solution;

impl Solution {
    pub fn minimum_operations_to_make_k_periodic(mut word: String, k: i32) -> i32 {
        let n = word.len();
        let k = k as usize;
        let mut cnt = std::collections::HashMap::new();
        for i in 1..=n / k {
            let s = word.split_off(n - i * k);
            cnt.entry(s).and_modify(|x| *x += 1).or_insert(1);
        }
        let mut res = 0;
        let mut max = 0;
        for &v in cnt.values() {
            if v > max {
                max = v;
            }
            res += v;
        }
        res - max
    }
}
