struct Solution;

impl Solution {
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        let mut vis = vec![false; 40_000];
        let mut cnt: Vec<(i32, i32, i32)> = apples.into_iter().zip(days).collect::<Vec<(i32, i32)>>().into_iter().enumerate().map(|(i, (a, d))| (i as i32 + d, i as i32, a)).collect();
        cnt.sort_unstable();
        let mut res = 0;
        for (e, s, a) in cnt {
            let mut num = 0;
            for i in s..e {
                if !vis[i as usize] {
                    vis[i as usize] = true;
                    num += 1;
                }
                if num >= a {
                    break;
                }
            }
            res += num;
        }
        res
    }
}