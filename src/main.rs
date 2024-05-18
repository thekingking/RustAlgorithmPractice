

fn main() {
    let mut powers = vec![];
    let mut n = 323243;
    while n != 0 {
        let lowbit = n & (-n);
        powers.push(lowbit);
        n ^= lowbit;
    }
    println!("{:?}", powers);
}

struct Solution;

impl Solution {
    
    pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const MOD:i64 = 1_000_000_007;
        let mut powers = vec![];
        let mut n = n;
        while n != 0 {
            let lowbit = n & (-n);
            powers.push(lowbit);
            n ^= lowbit;
        }
        let len = powers.len() as i32;
        let mut answer = vec![];
        for q in queries {
            let l = q[0].min(len - 1);
            let r = q[1].min(len - 1);
            let mut mul:i64 = 1;
            for i in l..=r {
                mul *= powers[i as usize] as i64;
                mul %= MOD;
            }
            answer.push(mul as i32);
        }
        answer
    }
}