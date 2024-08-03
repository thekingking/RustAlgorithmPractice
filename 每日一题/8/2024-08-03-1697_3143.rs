struct Solution;

impl Solution {
    /// 枚举
    pub fn max_points_inside_square(points: Vec<Vec<i32>>, s: String) -> i32 {
        let mut cnt = std::collections::BTreeMap::new();
        let bs = s.into_bytes();
        let n = points.len();
        for i in 0..n {
            let dis = std::cmp::max(points[i][0].abs(), points[i][1].abs());
            let bit = bs[i] - b'a';
            cnt.entry(dis).and_modify(|x| {
                if *x & 1 << bit != 0 {
                    *x |= 1 << 26;
                } else {
                    *x |= 1 << bit;
                }
            }).or_insert(1 << bit);
        }
        let mut sum = 0;
        let mut res = 0;
        for &v in cnt.values() {
            let num = (v as i32).count_ones() as i32;
            if sum & v != 0 || v & 1 << 26 != 0 {
                break;
            }
            res += num;
            sum |= v;
        }
        res
    }

    /// 最小次小
    pub fn max_points_inside_square(points: Vec<Vec<i32>>, s: String) -> i32 {
        let mut cnt = [[i32::MAX, i32::MAX]; 26];
        let bs = s.into_bytes();
        let n  = points.len();
        for i in 0..n {
            let dis = std::cmp::max(points[i][0].abs(), points[i][1].abs());
            let bit = (bs[i] - b'a') as usize;
            if cnt[bit][0] > dis {
                cnt[bit][1] = cnt[bit][0];
                cnt[bit][0] = dis;
            } else if cnt[bit][1] > dis {
                cnt[bit][1] = dis;
            }
        }
        let mut min = i32::MAX;
        for i in 0..26 {
            if min > cnt[i][1] {
                min = cnt[i][1];
            }
        }
        let mut res = 0;
        for i in 0..26 {
            if cnt[i][0] < min {
                res += 1;
            }
        }
        res
    }
}