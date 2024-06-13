use std::collections::HashMap;

struct Solution;

impl Solution {
    /// 反悔贪心，将profit排序，由最大到最小依次减小total_profits，增加distinct_categories，寻找最大answer
    pub fn find_maximum_elegance(mut items: Vec<Vec<i32>>, k: i32) -> i64 {
        items.sort_unstable_by_key(|x| x[0]);
        let mut profits = 0;
        let mut cnt = HashMap::new();
        let len = items.len() as i32;
        let mut i = len - 1;
        while i >= len - k {
            profits += items[i as usize][0] as i64;
            cnt.entry(items[i as usize][1]).and_modify(|x| *x += 1).or_insert(1);
            i -= 1;
        }
        let mut distinct_categories = cnt.len() as i64;
        let mut ans = profits + distinct_categories * distinct_categories;
        let mut j = i + 1;
        while i >= 0 && distinct_categories < k as i64 {
            if cnt.get(&items[i as usize][1]).is_none() {
                cnt.insert(items[i as usize][1], 1);
                while *cnt.get(&items[j as usize][1]).unwrap() == 1 {
                    j += 1;
                }
                cnt.entry(items[j as usize][1]).and_modify(|x| *x -= 1);
                profits = profits + items[i as usize][0] as i64 - items[j as usize][0] as i64;
                distinct_categories = cnt.len() as i64;
                ans = ans.max(profits + distinct_categories * distinct_categories);
                j += 1;
            }
            i -= 1;
        }
        ans
    }
}
