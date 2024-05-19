

fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let mut max = arr[0];
        let mut num = 0;
        for &i in &arr[1..] {
            if max < i {
                num = 0;
                max = i;
            }
            num += 1;
            if num == k {
                break;
            }
        }
        max
    }
}