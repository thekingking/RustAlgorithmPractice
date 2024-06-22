struct Solution;

impl Solution {
    /// 双百
    /// 前缀和+哈希表
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut cnt = std::collections::HashMap::new();
        let mut pre = 0;
        let mut res = 0;
        for (i, &x) in arr.iter().enumerate() {
            cnt.entry(pre).and_modify(|l: &mut Vec<usize>| l.push(i)).or_insert(vec![i]);
            pre ^= x;
            if cnt.contains_key(&pre) {
                let l = cnt.get(&pre).unwrap();
                res += l.len() * i - l.iter().sum::<usize>();
            }
        }
        res as i32
    }
}