struct Solution;

impl Solution {
    pub fn find_minimum_time(mut tasks: Vec<Vec<i32>>) -> i32 {
        let mut cnt = vec![0; 2002];
        tasks.sort_unstable_by_key(|x| x[1]);
        for row in tasks {
            let s = row[0] as usize;
            let mut e = row[1] as usize;
            let t = row[2];
            let &sum = &cnt[s..=e].iter().sum::<i32>();
            if sum < t {
                let mut r = t - sum;
                while e > 0 && r > 0 {
                    if cnt[e] != 1 {
                        cnt[e] = 1;
                        r -= 1;
                    }
                    e -= 1;
                }
            }
        }
        cnt.into_iter().sum()
    }  
}
