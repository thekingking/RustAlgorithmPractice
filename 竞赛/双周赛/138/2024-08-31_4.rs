struct Solution;

impl Solution {
    pub fn min_damage(power: i32, damage: Vec<i32>, health: Vec<i32>) -> i64 {
        use std::cmp::Ordering;
        let mut cnt: Vec<(i64, i64)> = damage.into_iter().map(|x| x as i64).zip(health.into_iter().map(|x| ((x + power - 1) / power) as i64)).collect();
        cnt.sort_unstable_by(|a, b| {
            if a.0 / a.1 > b.0 / b.1 {
                Ordering::Greater
            } else if a.0 / a.1 < b.0 / b.1 {
                Ordering::Less
            } else {
                let x = a.0 * b.1;
                let y = b.0 * a.1;
                if x > y {
                    Ordering::Greater
                } else if x < y {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            }
        });
        let mut res = 0;
        let mut sum_d = 0;
        for (d, t) in cnt {
            sum_d += d;
            res += (sum_d * t) as i64;
        }
        res
    }
}