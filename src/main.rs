
fn main() {
    println!("hello, world");
}

    struct Solution;

    impl Solution {
        pub fn latest_time_catch_the_bus(mut buses: Vec<i32>, mut passengers: Vec<i32>, capacity: i32) -> i32 {
            buses.sort_unstable();
            passengers.sort_unstable();

            // 模拟乘客上车
            let mut j = 0;
            let mut c = 0;
            for &t in &buses {
                c = capacity;
                while c > 0 && j < passengers.len() && passengers[j] <= t {
                    j += 1;
                    c -= 1;
                }
            }

            // 寻找插队时机
            j -= 1;
            let mut ans = if c > 0 { *buses.last().unwrap() } else { passengers[j] };
            while j < passengers.len() && ans == passengers[j] {
                ans -= 1; // 往前找没人到达的时刻
                j -= 1;
            }
            ans
        }
    }