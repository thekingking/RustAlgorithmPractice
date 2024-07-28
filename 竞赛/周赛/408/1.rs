struct Solution;

impl Solution {
    pub fn can_alice_win(nums: Vec<i32>) -> bool {
        let mut sum = 0;
        let mut cnt = 0;
        for x in nums {
            sum += x;
            if x >= 1 && x <= 9 {
                cnt += x;
            }
        }
        if cnt * 2 == sum {
            return false;
        }
        true
    }
}
