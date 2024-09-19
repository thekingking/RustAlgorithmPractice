struct Solution;

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;

        let mut cnt1 = BinaryHeap::new();
        let mut cnt2 = BinaryHeap::new();
        let n = profits.len();
        let mut ans = w;
        for i in 0..n {
            cnt1.push(Reverse((capital[i], profits[i])));
        }
        for _ in 0..k {
            while let Some(Reverse((c, p))) = cnt1.peek() {
                if *c <= ans {
                    cnt2.push(*p);
                    cnt1.pop();
                } else {
                    break;
                }
            }
            if let Some(p) = cnt2.pop() {
                ans += p;
            } else {
                return ans;
            }
        }
        ans
    }
}