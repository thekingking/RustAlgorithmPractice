fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];
        let mut pre = 0;
        let mut min_pre = 0;
        for n in nums {
            pre += n;
            ans = ans.max(pre - min_pre);
            min_pre = pre.min(min_pre);
        }
        ans
    }
}