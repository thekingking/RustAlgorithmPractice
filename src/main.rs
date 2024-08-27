
fn main() {
    println!("hello, world");
}

struct Solution;

impl Solution {
    pub fn median_of_uniqueness_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut cnt = Vec::new();
        let mut num = 1;
        for i in 0..n {
            if i == n - 1 || nums[i] != nums[i + 1] {
                cnt.push(num);
                num = 1;
            } else {
                num += 1;
            }
        }
        let n = n as i64;
        let t = n * (n + 1) / 4;
        let mut sum = n;
        let m = cnt.len();
        for i in 1..m {
            if sum >= t {
                return i as i32;
            }
            for j in 0..(m - i) {
                sum += cnt[j] * cnt[j + i];
            }
        }
        0
    }
}