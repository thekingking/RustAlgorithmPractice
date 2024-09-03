struct Solution;

impl Solution {
    pub fn max_strength(mut nums: Vec<i32>) -> i64 {
        let mut pos = Vec::new();
        let mut neg = Vec::new();
        let mut zero = 0;
        for x in nums {
            if x > 0 {
                pos.push(x);
            } else if x < 0 {
                neg.push(x);
            } else {
                zero += 1;
            }
        }
        if pos.len() == 0 && neg.len() <= 1 {
            if zero > 0 {
                return 0;
            } else {
                return neg[0] as i64;
            }
        }
        let mut res = 1;
        for x in pos {
            res *= x as i64;
        }
        let mut max = -10;
        for x in neg {
            res *= x as i64;
            max = max.max(x as i64);
        }
        if res < 0 {
            res /= max;
        }
        res
    }
}