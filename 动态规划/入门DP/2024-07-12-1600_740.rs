struct Solution;

impl Solution {
    /// 排序 + 动态规划
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::BTreeMap::new();
        for x in nums {
            map.entry(x).and_modify(|x| *x += 1).or_insert(1);
        }
        let mut pre = 0;
        let mut f1 = 0;
        let mut f2 = 0;
        for (&k, &v) in map.iter().rev() {
            let f3 = if k == pre {
                std::cmp::max(f2, f1 + k * v)
            } else {
                f2 + k * v
            };
            f1 = f2;
            f2 = f3;
            pre = k - 1;
        }
        f2
    }
}