struct Solution;

impl Solution {
    /// 贪心，先排序，然后尽可能多的在船上载人
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();
        let mut ans = 0;
        let mut left = 0;
        let mut right = people.len() - 1;
        while left < right {
            if people[left] + people[right] <= limit {
                left += 1;
            }
            right -= 1;
            ans += 1;
        }
        if left == right {
            ans += 1;
        }
        ans
    }
}
