struct Solution;

impl Solution {
    pub fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        let is_prime = [0,
            0, 1, 1, 0, 1, 0, 1, 0, 0, 0,
            1, 0, 1, 0, 0, 0, 1, 0, 1, 0,
            0, 0, 1, 0, 0, 0, 0, 0, 1, 0,
            1, 0, 0, 0, 0, 0, 1, 0, 0, 0,
            1, 0, 1, 0, 0, 0, 1, 0, 0, 0,
            0, 0, 1, 0, 0, 0, 0, 0, 1, 0,
            1, 0, 0, 0, 0, 0, 1, 0, 0, 0,
            1, 0, 1, 0, 0, 0, 0, 0, 1, 0,
            0, 0, 1, 0, 0, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
        ];
        while left < nums.len() {
            if is_prime[nums[left] as usize] == 1 {
                break;
            }
            left += 1;
        }
        while right >= 0 {
            if is_prime[nums[right as usize] as usize] == 1 {
                break;
            }
            right -= 1;
        }
        right - left as i32
    }
}