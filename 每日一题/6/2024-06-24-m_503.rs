struct Solution;

impl Solution {
    /// 模拟
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len()];
        let max = *nums.iter().max().unwrap();
        let len = nums.len();
        for (i, &x) in nums.iter().enumerate() {
            if x == max {
                res[i] = -1;
                continue;
            }
            let mut j = (i + 1) % len;
            while j != i && nums[j] <= x {
                j = (j + 1) % len;
            }
            res[i] = nums[j];
        }
        res
    }

    /// 单调栈、从右向左，以nums中元素作为栈中元素
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![-1; n];
        let mut stack = Vec::new();
        for i in (0..2 * n).rev() {
            let x = nums[i % n];
            while let Some(&top) = stack.last() {
                if x < top {
                    break;
                }
                stack.pop();
            }
            if i < n && !stack.is_empty() {
                res[i] = *stack.last().unwrap();
            }
            stack.push(x);
        }
        res
    }

    /// 单调栈、从左向右，以nums元素下标为stack中元素
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![-1; n];
        let mut stack = Vec::new();
        for i in 0..2 * n {
            let x = nums[i % n];
            while let Some(&top) = stack.last() {
                if x <= nums[top] {
                    break;
                }
                res[top] = x;
                stack.pop();
            }
            if i < n {
                stack.push(i);
            }
        }
        res
    }
}