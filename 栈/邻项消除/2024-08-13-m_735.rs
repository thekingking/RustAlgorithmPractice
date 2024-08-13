struct Solution;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        for x in asteroids {
            let mut last = 0;
            while let Some(&pre) = res.last() {
                if pre > 0 && x < 0 {
                    last = pre;
                    res.pop();
                    if x.abs() <= pre {
                        break;
                    }
                } else {
                    break;
                }
            }
            if last > 0 && x < 0 {
                if last > x.abs() {
                    res.push(last);
                } else if last < x.abs() {
                    res.push(x);
                }
            } else {
                res.push(x);
            }
        }
        res
    }
}