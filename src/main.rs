

fn main() {
    println!("hello, world");
    Solution::max_coins(vec![3,1,5,8]);
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut f1 = 0;
        let mut f2 = 0;
        for x in nums {
            let f3 = std::cmp::max(f2, f1 + x);
            f1 = f2;
            f2 = f3;
        }
        f2
    }
}
