struct Solution;

impl Solution {
    pub fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;

        let mut buy: BinaryHeap<(i32, i32)> = BinaryHeap::new();
        let mut sell: BinaryHeap<(i32, i32)> = BinaryHeap::new();
        for order in orders {
            let mut amount = order[1];
            if order[2] == 0 {
                while let Some(&(p, a)) = sell.peek() {
                    if -p <= order[0] {
                        if a < amount {
                            sell.pop();
                            amount -= a;
                        } else if a > amount {
                            sell.peek_mut().unwrap().1 -= amount;
                            amount = 0;
                            break;
                        } else {
                            sell.pop();
                            amount = 0;
                            break;
                        }
                    } else {
                        break;
                    }
                }
                if amount > 0 {
                    buy.push((order[0], amount));
                }
            } else {
                while let Some(&(p, a)) = buy.peek() {
                    if p >= order[0] {
                        if a < amount {
                            buy.pop();
                            amount -= a;
                        } else if a > amount {
                            buy.peek_mut().unwrap().1 -= amount;
                            amount = 0;
                            break;
                        } else {
                            buy.pop();
                            amount = 0;
                            break;
                        }
                    } else {
                        break;
                    }
                }
                if amount > 0 {
                    sell.push((-order[0], amount));
                }
            }
        }
        let mut res = 0;
        for &(_, a) in buy.iter() {
            res = (res + a) % 1_000_000_007;
        }
        for &(_, a) in sell.iter() {
            res = (res + a) % 1_000_000_007;
        }
        res
    }
}