struct Solution;

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        for i in 0..tickets.len() {
            if i > k as usize {
                res += tickets[i].min(tickets[k as usize] - 1);
            } else {
                res += tickets[i].min(tickets[k as usize]);
            }
        }
        res
    }
}
