struct Solution;

impl Solution {
    /// 中心前缀和 + 哈希表，从k向左右求前缀和，以哈希表存储值
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = HashMap::new();
        let mut index = 0;
        let mut ans = 0;
        for i in 0..nums.len() {
            if k == nums[i] {
                index = i;
                break;
            }
        }
        let mut i = index as i32;
        let mut sum = 0;
        loop {
            cnt.entry(sum).and_modify(|x| *x += 1).or_insert(1);
            i -= 1;
            if i < 0 {
                break;
            }
            sum += if nums[i as usize] >= k { -1 } else { 1 };
        }
        let mut j = index;
        let mut sum = 0;
        loop {
            ans += cnt.get(&sum).unwrap_or(&0) + cnt.get(&(sum - 1)).unwrap_or(&0);
            j += 1;
            if j >= nums.len() {
                break;
            }
            sum += if nums[j] >= k { 1 } else { -1 };
        }
        ans
    }
}
