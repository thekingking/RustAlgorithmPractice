struct Solution;

impl Solution {
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        fn gcd(x: i32, y: i32) -> i32 {
            let mut a = x;
            let mut b = y;
            let mut c = a % b;
            while c != 0 {
                a = b;
                b = c;
                c = a % b;
            }
            b
        }
        let mut stack = vec![nums[0]];
        for &x in &nums[1..] {
            let mut cur = x;
            while stack.len() > 0 && gcd(*stack.last().unwrap(), cur) != 1 {
                let pre = stack.pop().unwrap();
                cur = cur / gcd(cur, pre) * pre;
            }
            stack.push(cur);
        }
        stack
    }
}