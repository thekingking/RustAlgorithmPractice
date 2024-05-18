

fn main() {
    println!("{}", false as i32);
}

struct Solution;

impl Solution {
    
    pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
        let mut max = -1;
        let mut div = 0;
        for i in 0..divisors.len() {
            let mut cur = 0;
            for j in 0..nums.len() {
                if nums[j] % divisors[i] == 0 {
                    cur += 1;
                }
            }
            if cur > max || cur == max && div > divisors[i] {
                div = divisors[i];
                max = cur;
            }
        }
        div
    }
}