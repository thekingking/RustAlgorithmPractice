struct Solution;

impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut cnt = vec![0; 1001];
        let mut sum = 0;
        for t in trips {
            cnt[t[1] as usize] += t[0];
            cnt[t[2] as usize] -= t[0];
        }
        for c in cnt {
            sum += c;
            if sum > capacity {
                return false;
            }
        }
        true
    }
}