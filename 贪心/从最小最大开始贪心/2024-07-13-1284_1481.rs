struct Solution;

impl Solution {
    /// 贪心 + 哈希
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        let mut cnt = std::collections::HashMap::new();
        for x in arr {
            cnt.entry(x).and_modify(|x| *x += 1).or_insert(1);
        }
        let mut num = cnt.len() as i32;
        let mut bt = std::collections::BTreeMap::new();
        for (_, v) in cnt {
            bt.entry(v).and_modify(|x| *x += 1).or_insert(1);
        }
        let mut n = k;
        for (k, v) in bt {
            if n - k * v <= 0 {
                num -= n / k;
                return num;
            }
            n -= k * v;
            num -= v;
        }
        0
    }
}