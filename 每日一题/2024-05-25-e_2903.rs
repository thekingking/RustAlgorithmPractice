struct Solution;

impl Solution {
    // 暴力，两重循环，O(n2)
    pub fn find_indices1(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in (i + index_difference as usize)..nums.len() {
                if (nums[j] - nums[i]).abs() >= value_difference {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![-1, -1]
    }

    // 官解，一次遍历，O(n)，双百
    pub fn find_indices2(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        let mut max = 0;
        let mut min = 0;
        for j in (index_difference as usize)..nums.len() {
            let i = j - index_difference as usize;
            if nums[min] > nums[i] {
                min = i;
            }
            if nums[j] - nums[min] >= value_difference {
                return vec![min as i32, j as i32];
            }
            if nums[max] < nums[i] {
                max = i;
            }
            if nums[max] - nums[j] >= value_difference {
                return vec![max as i32, j as i32];
            }
        }
        vec![-1, -1]
    }
}