struct Solution;

impl Solution {
    /// 暴力
    pub fn distribute_candies(mut candies: i32, num_people: i32) -> Vec<i32> {
        let mut ans = vec![0; num_people as usize];
        let mut i = 1;
        while candies > 0 {
            ans[((i - 1) % num_people) as usize] += candies.min(i);
            candies -= candies.min(i);
            i += 1;
        }
        ans
    }
}