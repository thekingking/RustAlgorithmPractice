struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut min = 0;
        let mut min_index = 0;
        let n = gas.len();
        let mut sum = 0;
        for i in 0..n {
            sum += gas[i] - cost[i];
            if sum < min {
                min = sum;
                min_index = i as i32 + 1;
            }
        }
        if sum < 0 {
            return -1;
        }
        min_index
    }
}