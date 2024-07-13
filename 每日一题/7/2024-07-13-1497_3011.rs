struct Solution;

impl Solution {
    /// 一次遍历
    pub fn can_sort_array(mut nums: Vec<i32>) -> bool {
        let mut cur_max = 0;
        let mut pre_max = 0;
        let mut num = 0;
        for x in nums {
            let mut n = 0;
            let mut t = x;
            while t > 0 {
                if t % 2 == 1 {
                    n += 1;
                }
                t /= 2;
            }
            if n != num {
                pre_max = cur_max;
                num = n;
            }
            if x < pre_max {
                return false;
            }
            if x > cur_max {
                cur_max = x;
            }
        }
        true
    }
}