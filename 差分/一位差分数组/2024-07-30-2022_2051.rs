struct Solution;

impl Solution {
    /// 差分，排序
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let mut cnt = std::collections::BTreeMap::new();
        for flower in flowers {
            cnt.entry(flower[0]).and_modify(|x| *x += 1).or_insert(1);
            cnt.entry(flower[1] + 1).and_modify(|x| *x -= 1).or_insert(-1);
        }
        let mut people: Vec<(i32, usize)> = std::iter::zip(people, 0..).collect();
        people.sort_unstable_by_key(|x| x.0);
        let mut sum = 0;
        let mut res = vec![0; people.len()];
        let mut i = 0;
        for (k, v) in cnt {
            while i < people.len() && people[i].0 < k {
                res[people[i].1] = sum;
                i += 1;
            }
            sum += v;
        }
        while i < people.len() {
            res[people[i].1] = sum;
            i += 1;
        }
        res
    }
}
