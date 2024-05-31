use std::collections::HashMap;

struct Solution;

impl Solution {
    /// 题目要求空间复杂度O(1)，但是我不会
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut cnt = HashMap::new();
        for x in nums {
            cnt.entry(x).and_modify(|x| *x += 1).or_insert(1);
        }
        for (k, v) in cnt {
            if v == 1 {
                ans.push(k);
            }
        }
        ans
    }

    /// 灵神的题解，异或和计算
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let xor_all = nums.iter().fold(0, |xor, &x| xor ^ x);
        let low_bit = xor_all & -xor_all;
        let mut ans = vec![0, 0];
        for x in nums {
            if x & low_bit == 0 {
                ans[0] ^= x;
            } else {
                ans[1] ^= x;
            }
        }
        ans
    }
}