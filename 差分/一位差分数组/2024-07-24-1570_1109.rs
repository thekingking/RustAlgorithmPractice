struct Solution;

impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut res = vec![0; n + 1];
        for booking in bookings {
            res[booking[0] as usize - 1] += booking[2];
            res[booking[1] as usize] -= booking[2];
        }
        let mut sum = 0;
        for i in 0..n {
            sum += res[i];
            res[i] = sum;
        }
        res.pop();
        res
    }
}
