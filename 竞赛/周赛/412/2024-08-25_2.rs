struct Solution;

impl Solution {
    pub fn count_pairs(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        use std::collections::HashSet;

        let mut cnt = HashMap::new();
        for &x in &nums {
            cnt.entry(x).and_modify(|x| *x += 1).or_insert(1);
        }
        let mut res = 0;
        for &x in &nums {
            cnt.entry(x).and_modify(|x| *x -= 1).or_insert(0);
            let mut set = HashSet::new();
            set.insert(x);
            let mut s = vec![0; 6];
            let mut x = x;
            res += *cnt.get(&x).unwrap_or(&0);
            for i in (0..6).rev() {
                s[i] = x % 10;
                x /= 10;
            }
            for i in 0..6 {
                for j in (i + 1)..6 {
                    s.swap(i, j);
                    let mut n = 0;
                    for k in 0..6 {
                        n = n * 10 + s[k];
                    }
                    if !set.contains(&n) {
                        set.insert(n);
                        res += *cnt.get(&n).unwrap_or(&0);
                    }
                    s.swap(i, j);
                }
            }
        }
        res
    }
}