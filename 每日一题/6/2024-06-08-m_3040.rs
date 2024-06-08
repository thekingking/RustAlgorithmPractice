use std::collections::HashMap;

struct Solution;

impl Solution {
    /// 双百
    /// 记忆化搜索，暴力枚举每种情况时把当前状态存储在哈希表中，后面遇到相同情况直接拿出来用就行了
    /// 当某种情况遍历完整个数组即表示这个为最佳解，也没必要继续求解了，所以添加了done进行判断
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        fn search(nums: &Vec<i32>, cnt: &mut HashMap<(i32, i32), i32>, left: i32, right: i32, target: i32, done: &mut bool) -> i32 {
            if right <= left || *done {
                *done = true;
                return 0;
            }
            if cnt.get(&(left, right)).is_none() {
                let mut res = 0;
                res = res.max(if nums[left as usize] + nums[left as usize + 1] == target {
                    search(nums, cnt, left + 2, right, target, done) + 1
                } else { 0 });
                res = res.max(if nums[left as usize] + nums[right as usize] == target {
                    search(nums, cnt, left + 1, right - 1, target, done) + 1
                } else { 0 });
                res = res.max(if nums[right as usize] + nums[right as usize - 1] == target {
                    search(nums, cnt, left, right - 2, target, done) + 1
                } else { 0 });
                cnt.insert((left, right), res);
            }
            return *cnt.get(&(left, right)).unwrap();
        }

        let mut done = false;
        let left = 0;
        let right = nums.len() as i32 - 1;

        let mut cnt1: HashMap<(i32, i32), i32> = HashMap::new();
        let mut cnt2: HashMap<(i32, i32), i32> = HashMap::new();
        let mut cnt3: HashMap<(i32, i32), i32> = HashMap::new();

        let num1 = search(&nums, &mut cnt1, left + 2, right, nums[left as usize] + nums[left as usize + 1], &mut done);
        let num2 = search(&nums, &mut cnt2, left + 1, right - 1, nums[left as usize] + nums[right as usize], &mut done);
        let num3 = search(&nums, &mut cnt3, left, right - 2, nums[right as usize] + nums[right as usize - 1], &mut done);

        1 + num1.max(num2.max(num3))
    }
}
