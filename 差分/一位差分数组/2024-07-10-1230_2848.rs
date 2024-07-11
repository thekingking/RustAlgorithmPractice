struct Solution;

impl Solution {
    /// 暴力枚举
    pub fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
        let mut cnt = vec![0; 100];
        for row in nums {
            for x in row[0]..=row[1] {
                cnt[x as usize - 1] = 1;
            }
        }
        cnt.iter().sum()
    }

    /// 差分数组
    pub fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
        let mut cnt = vec![0; 102];
        for p in nums {
            cnt[p[0] as usize] += 1;
            cnt[p[1] as usize + 1] -= 1;
        }

        let mut ans = 0;
        let mut s = 0;
        for d in cnt {
            s += d;
            if s > 0 {
                ans += s;
            }
        }
        ans
    }
}